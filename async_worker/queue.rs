use dashmap::DashMap;
use log::error;
use std::{sync::Arc, time::Duration};
use tokio::{
	select,
	sync::{mpsc, oneshot},
	time::sleep,
};

use crate::{
	handle::{NoopHandle, WorkerHandle},
	worker::{drive_workers, InternalPollResponse, PeerRequest, SpawnMessage, TaskMessage, Worker},
	Error, Result,
};

struct QueueOptions {
	max_length: usize,
	terminate_worker_after: Duration,
}

impl Default for QueueOptions {
	fn default() -> Self {
		QueueOptions {
			max_length: 5,
			terminate_worker_after: Duration::from_secs(60 * 20),
		}
	}
}

#[derive(Default)]
pub struct QueueBuilder {
	options: QueueOptions,
}

impl QueueBuilder {
	/// The maxium length of the queue for a given worker. If `length` tasks are enqueued before the worker has an opportunity to get to them,
	/// the `length + 1` enqueue call will yield `EnqueueResult::WorkerAtCapacity`
	///
	/// It is not just `Queue::enqueue` that is held to this standard. `Queue::attach_waiter`, `Queue::poll`, and `Queue::poll_until` will not
	/// perform their respective operations if the queue is full. Additionally, if a single `Worker::handle` call is taking a long time, the
	/// aforementinoed operations contribute to the queue reaching it's max length.
	pub fn max_length(mut self, length: usize) -> QueueBuilder {
		self.options.max_length = length;

		self
	}

	/// The amount of inactivity after which a worker is automatically terminated. Defaults to 20 minutes.
	///
	/// NOTE: polling is considered to be activity
	pub fn terminate_worker_after(mut self, duration: Duration) -> QueueBuilder {
		self.options.terminate_worker_after = duration;

		self
	}

	pub fn build<W, H>(self, context: W::Context) -> Queue<W, H>
	where
		W: Worker + Send + 'static,
		H: WorkerHandle<W::Request, W::Response> + Send + 'static,
	{
		Queue::new(self.options, context)
	}
}

#[derive(Debug)]
pub struct Queue<W: Worker, H: WorkerHandle<W::Request, W::Response> = NoopHandle> {
	max_length: usize,
	spawn_sender: mpsc::Sender<SpawnMessage<W, H>>,
	peer_requests_sender: mpsc::Sender<Vec<PeerRequest<W::Id, W::Request>>>,
	map: Arc<DashMap<W::Id, mpsc::Sender<TaskMessage<W::Request, W::Response, H>>>>,
	context: W::Context,
}

impl<W, H> Queue<W, H>
where
	W: Worker + Send + 'static,
	H: WorkerHandle<W::Request, W::Response> + Send + 'static,
{
	fn new(options: QueueOptions, context: W::Context) -> Queue<W, H> {
		let (spawn_sender, spawn_receiver) = mpsc::channel(1000);
		let (peer_requests_sender, mut peer_requests_receiver) = mpsc::channel(1000);

		let queue = Queue::<W, H> {
			max_length: options.max_length,
			spawn_sender,
			peer_requests_sender,
			map: Arc::new(DashMap::new()),
			context,
		};
		let copied_queue = queue.clone();

		tokio::spawn(async move { drive_workers(options.terminate_worker_after, spawn_receiver).await });
		tokio::spawn(async move {
			loop {
				let peer_requests = match peer_requests_receiver.recv().await {
					Some(requests) => requests,
					None => break,
				};

				for request in peer_requests {
					copied_queue.enqueue(&request.id, request.content);
				}
			}
		});

		queue
	}

	/// Attach a waiter to the worker referenced by `id`. Waiters always take precident over polling, so if there is an active waiter, all responses will be immediately
	/// piped to it and poll calls will hang until the the next response after the waiter is dropped.
	pub fn register_handle(&self, id: &W::Id, handle: H) -> Result<()> {
		let message = TaskMessage::RegisterHandle { handle };

		let send_res = {
			match self.map.get(id) {
				Some(sender) => sender.try_send(message),
				None => return Err(Error::NoWorker),
			}
		};

		match send_res {
			Err(mpsc::error::TrySendError::Full(_)) => return Err(Error::WorkerAtCapacity),
			Err(mpsc::error::TrySendError::Closed(_)) => {
				self.map.remove(id);

				return Err(Error::NoWorker);
			}
			_ => (),
		}

		Ok(())
	}

	/// Enqueue a new request for the worker referenced by `id` to pick up. The response can be retrived by either polling or attaching a handle.
	///
	/// If a worker does not exist for this id, a new worker will be created.
	///
	/// Once the worker is ready for a new task, `Worker::handle` will be called with this `request`.
	pub async fn enqueue(&self, id: &W::Id, request: W::Request) -> Result<()> {
		enum Action<Request> {
			Close,
			None,
			Spawn(Request),
		}

		let action = {
			match self.map.get(id) {
				Some(sender) => match sender.try_send(TaskMessage::Enqueue { request }) {
					Err(mpsc::error::TrySendError::Full(_)) => return Err(Error::WorkerAtCapacity),
					Err(mpsc::error::TrySendError::Closed(_)) => Action::Close, // we return close instead of closing right away in order to prevent deadlock
					_ => Action::None,
				},
				None => Action::Spawn(request),
			}
		};

		if let Action::Close = action {
			self.map.remove(id);

			return Err(Error::NoWorker);
		} else if let Action::Spawn(request) = action {
			let (sender, receiver) = mpsc::channel(self.max_length);
			let id_to_insert = id.clone();

			// We want to do as little as possible in here because it will keep a mutex locked
			{
				self.map.insert(id_to_insert, sender);
			}

			let send_res = self
				.spawn_sender
				.send(SpawnMessage {
					id: id.clone(),
					context: self.context.clone(),
					message_receiver: receiver,
					initial_request: request,
					peer_requests_sender: self.peer_requests_sender.clone(),
				})
				.await;

			if let Err(_) = send_res {
				error!("The spawning task was closed, which should only happen when this object is dropped. It wasn't dropped though, because we are using it");
			}
		}

		Ok(())
	}

	/// Poll for the next worker response. If there is already a waiting poll, the ongoing poll will immediately throw an `Error::Ceeded`.
	///
	/// This function will only poll the worker that is referenced by `id`, and if such a worker does not exist, an `Error::NoWorker` will be thrown.
	///
	/// If the worker is terminated while the poll is ongoing, an `Error::WorkerTerminated` will be thrown.
	///
	/// If there is an open handle, this function will throw an `Error::Ceeded`
	pub async fn poll(&self, id: &W::Id) -> Result<W::Response> {
		let (responder, receiver) = oneshot::channel();
		let message = TaskMessage::Poll { responder };

		let send_res = {
			let task_sender = match self.map.get(id) {
				Some(sender) => sender,
				None => return Err(Error::NoWorker),
			};

			task_sender.try_send(message)
		};

		match send_res {
			Err(mpsc::error::TrySendError::Full(_)) => return Err(Error::WorkerAtCapacity),
			Err(mpsc::error::TrySendError::Closed(_)) => {
				self.map.remove(id);

				return Err(Error::NoWorker);
			}
			_ => (),
		};

		let response = match receiver.await {
			Ok(InternalPollResponse::Ok(response)) => response,
			Ok(InternalPollResponse::Ceeded) => return Err(Error::NoWorker),
			Ok(InternalPollResponse::WorkerTerminated) => return Err(Error::WorkerTerminated),
			Err(_) => return Err(Error::NoWorker),
		};

		Ok(response)
	}

	/// Poll for the next worker responses. If there is already an ongoing poll, the ongoing poll will immediately throw `Error::Ceeded`.
	///
	/// This function will only poll the worker that is referenced by `id`, and if such a worker does not exist, an `Error::NoWorker` will be thrown.
	///
	/// If the worker is terminated while the poll is ongoing, an `Error::WorkerTerminated` will be thrown.
	///
	/// If there is an open handle, this function throw an `Error::Ceeded`
	pub async fn poll_many(&self, id: &W::Id) -> Result<Vec<W::Response>> {
		let (responder, receiver) = oneshot::channel();
		let message = TaskMessage::PollMany { responder };

		let send_res = {
			let task_sender = match self.map.get(id) {
				Some(sender) => sender,
				None => return Err(Error::NoWorker),
			};

			task_sender.try_send(message)
		};

		match send_res {
			Err(mpsc::error::TrySendError::Full(_)) => return Err(Error::WorkerAtCapacity),
			Err(mpsc::error::TrySendError::Closed(_)) => {
				self.map.remove(id);

				return Err(Error::NoWorker);
			}
			_ => (),
		};

		let response = match receiver.await {
			Ok(InternalPollResponse::Ok(response)) => response,
			Ok(InternalPollResponse::Ceeded) => return Err(Error::NoWorker),
			Ok(InternalPollResponse::WorkerTerminated) => return Err(Error::WorkerTerminated),
			Err(_) => return Err(Error::NoWorker),
		};

		Ok(response)
	}

	/// Poll for the next responses, but timeout after `duration`. If there is already an ongoing poll, the ongoing poll will immediately throw
	/// `Error::Ceeded`
	/// This function will only poll the worker that is referenced by `id`, and if such a worker does not exist, an `Error::NoWorker` will be thrown.
	///
	/// If the worker is terminated while the poll is ongoing, an `Error::WorkerTerminated` will be thrown.
	///
	pub async fn poll_while(&self, id: &W::Id, duration: Duration) -> Result<W::Response> {
		select! {
			result = self.poll(id) => result,
			_ = sleep(duration) => Err(Error::Timeout)
		}
	}

	/// Poll for the next response, but timeout after `duration`. If there is already an ongoing poll, the ongoing poll will immediately throw
	/// `Error::Ceeded`
	/// This function will only poll the worker that is referenced by `id`, and if such a worker does not exist, an `Error::NoWorker` will be thrown.
	///
	/// If the worker is terminated while the poll is ongoing, an `Error::WorkerTerminated` will be thrown.
	///
	pub async fn poll_many_while(&self, id: &W::Id, duration: Duration) -> Result<Vec<W::Response>> {
		select! {
			result = self.poll_many(id) => result,
			_ = sleep(duration) => Err(Error::Timeout)
		}
	}

	/// Terminate a specific worker. If a handle is attached to it, it will be closed after a call to `WorkerHandle::will_drop` with a `DropReason::WorkerTerminated`.
	/// Then, `Worker::destroy()` will called.
	///
	/// This function does not guarantee that the worker is instantly terminated. Instead, is queues a termination to be performed once the async runtime and worker
	/// have capacity to perform the termination. In other words, this queues a graceful termination instead of forcefully shutting down the worker.
	pub fn terminate(&self, id: &W::Id) {
		self.map.remove(id);
	}
}

impl<W, H> Clone for Queue<W, H>
where
	W: Worker,
	H: WorkerHandle<W::Request, W::Response>,
{
	fn clone(&self) -> Self {
		Queue {
			max_length: self.max_length,
			spawn_sender: self.spawn_sender.clone(),
			peer_requests_sender: self.peer_requests_sender.clone(),
			map: self.map.clone(),
			context: self.context.clone(),
		}
	}
}
