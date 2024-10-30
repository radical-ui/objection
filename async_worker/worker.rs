use log::{debug, info};
use publisher::Publisher;
use std::{collections::VecDeque, fmt::Debug, future::Future, hash::Hash, time::Duration};
use tokio::{
	select,
	sync::{mpsc, oneshot},
	time::sleep,
};

use crate::handle::{recv_from_handle, DropReason, SendResult, WorkerHandle};

pub enum InternalPollResponse<T> {
	Ok(T),
	Ceeded,
	WorkerTerminated,
}

pub enum TaskMessage<Request: Sized, Response: Sized, Handle: WorkerHandle<Request, Response>> {
	Poll {
		responder: oneshot::Sender<InternalPollResponse<Response>>,
	},
	PollMany {
		responder: oneshot::Sender<InternalPollResponse<Vec<Response>>>,
	},
	RegisterHandle {
		handle: Handle,
	},
	Enqueue {
		request: Request,
	},
}

pub struct SpawnMessage<W: Worker, Handle: WorkerHandle<W::Request, W::Response>> {
	pub id: W::Id,
	pub context: W::Context,
	pub message_receiver: mpsc::Receiver<TaskMessage<W::Request, W::Response, Handle>>,
	pub initial_request: W::Request,
	pub peer_requests_sender: mpsc::Sender<Vec<PeerRequest<W::Id, W::Request>>>,
}

#[derive(Debug)]
pub struct PeerRequest<Id, Content> {
	pub id: Id,
	pub content: Content,
}

async fn send_peer_requests<Id, Request>(sender: &mpsc::Sender<Vec<PeerRequest<Id, Request>>>, requests: Vec<PeerRequest<Id, Request>>) {
	if !requests.is_empty() {
		let _ = sender.send(requests).await;
	}
}

pub trait Worker
where
	Self: Sized + Send,
{
	type Context: 'static + Send + Sized + Clone;
	type Request: 'static + Send + Sized;
	type Response: 'static + Send + Sized;
	type Id: 'static + Hash + PartialOrd + Eq + Clone + Send + Debug + Sync;

	/// Creates a new worker, which will be referenced to by the queue as `id`. `context` is a clone of the context that was given to the queue when it was built
	fn create(id: &Self::Id, context: Self::Context) -> impl Future<Output = Self> + Send;

	/// Handle a new response. The output will be able to be attained by the queue via a `WorkerHandle` or polling.
	fn handle(
		&mut self,
		request: Self::Request,
		peer_requests: &Publisher<PeerRequest<Self::Id, Self::Request>>,
	) -> impl Future<Output = Self::Response> + Send;

	/// Called just before this worker is dropped, but after the worker handle (if present) was dropped and any ongoing polls were closed with an `Error::WorkerTerminated`.
	fn destroy(self) -> impl Future<Output = ()> + Send;
}

pub async fn drive_workers<W, H>(worker_inactivity_timeout: Duration, mut spawn_receiver: mpsc::Receiver<SpawnMessage<W, H>>)
where
	W: Worker,
	H: WorkerHandle<W::Request, W::Response> + Send + 'static,
{
	loop {
		let SpawnMessage {
			id,
			context,
			mut message_receiver,
			initial_request,
			peer_requests_sender,
		} = match spawn_receiver.recv().await {
			Some(message) => message,
			None => break,
		};

		tokio::spawn(async move {
			let mut worker = W::create(&id, context).await;
			let mut response_list = VecDeque::<W::Response>::new();
			let mut stashed_handle = Option::<H>::None;
			let mut single_response_sender = Option::<ResponseSender<W::Response>>::None;

			let peer_requests = Publisher::new();
			response_list.push_back(worker.handle(initial_request, &peer_requests).await);
			send_peer_requests(&peer_requests_sender, peer_requests.items()).await;

			loop {
				let message = select! {
					message = message_receiver.recv() => match message {
						Some(message) => message,
						None => {
							debug!("worker {id:?} was manually terminated");

							break
						},
					},
					message = recv_from_handle::<W, H>(stashed_handle.as_mut()) => match message {
						Some(request) => TaskMessage::Enqueue { request },
						None => {
							debug!("worker {id:?} just had it's handle close");

							stashed_handle.unwrap().will_drop(DropReason::HandleClosed).await;
							stashed_handle = None;

							continue
						}
					},
					_ = sleep(worker_inactivity_timeout) => {
						debug!("task {id:?} was terminated due to an inactivity timeout of {}s", worker_inactivity_timeout.as_secs());

						break
					},
				};

				match message {
					TaskMessage::Poll { responder } => {
						if let None = stashed_handle {
							match response_list.pop_front() {
								Some(waiting_response) => {
									if let Err(rejected) = responder.send(InternalPollResponse::Ok(waiting_response)) {
										response_list.push_front(match rejected {
											InternalPollResponse::Ok(rejected) => rejected,
											InternalPollResponse::Ceeded => unreachable!(),
											InternalPollResponse::WorkerTerminated => unreachable!(),
										});
									}
								}
								None => {
									if let Some(old_responder) = single_response_sender.replace(ResponseSender::Single(responder)) {
										let _ = old_responder.send_new(InternalPollResponse::Ceeded);
									}
								}
							}
						} else {
							// drop the responder without sending if there is already a listening waiter
							let _ = responder.send(InternalPollResponse::Ceeded);
						}
					}
					TaskMessage::PollMany { responder } => {
						if let None = stashed_handle {
							let waiting_responses = response_list.drain(..).collect::<Vec<_>>();

							if !waiting_responses.is_empty() {
								if let Err(rejected) = responder.send(InternalPollResponse::Ok(waiting_responses)) {
									response_list.extend(match rejected {
										InternalPollResponse::Ok(rejected) => rejected,
										InternalPollResponse::Ceeded => unreachable!(),
										InternalPollResponse::WorkerTerminated => unreachable!(),
									});
								}
							} else {
								if let Some(old_responder) = single_response_sender.replace(ResponseSender::Many(responder)) {
									let _ = old_responder.send_new(InternalPollResponse::Ceeded);
								}
							}
						} else {
							let _ = responder.send(InternalPollResponse::Ceeded);
						}
					}
					TaskMessage::RegisterHandle { mut handle } => {
						let mut did_close = false;

						loop {
							let waiting_message = match response_list.pop_front() {
								Some(response) => response,
								None => break,
							};

							match handle.send(waiting_message).await {
								SendResult::Sent => (),
								SendResult::Closed(rejected) => {
									did_close = true;
									response_list.push_front(rejected);
								}
								SendResult::Failed(rejected) => response_list.push_front(rejected),
							};
						}

						if !did_close {
							debug!("adding a new handle for worker {id:?}");

							if let Some(mut old_handle) = stashed_handle.replace(handle) {
								old_handle.will_drop(DropReason::Ceeded).await;
							}
						}
					}
					TaskMessage::Enqueue { request } => {
						let peer_requests = Publisher::new();
						let mut response = worker.handle(request, &peer_requests).await;
						send_peer_requests(&peer_requests_sender, peer_requests.items()).await;

						if let Some(handle) = &mut stashed_handle {
							loop {
								match handle.send(response).await {
									SendResult::Sent => break,
									SendResult::Closed(rejected) => {
										handle.will_drop(DropReason::HandleClosed).await;
										stashed_handle = None;

										response_list.push_back(rejected);
										break;
									}
									SendResult::Failed(rejected) => response = rejected,
								};
							}
						} else if let Some(sender) = single_response_sender.take() {
							if let Err(rejected) = sender.send_new(InternalPollResponse::Ok(response)) {
								response_list.push_back(match rejected {
									InternalPollResponse::Ok(rejected) => rejected,
									InternalPollResponse::Ceeded => unreachable!(),
									InternalPollResponse::WorkerTerminated => unreachable!(),
								});
								single_response_sender = None;
							}
						} else {
							response_list.push_back(response);
						}
					}
				}
			}

			info!("running destructors for task {id:?}");

			if let Some(handle) = &mut stashed_handle {
				handle.will_drop(DropReason::WorkerTerminated).await;
			}
			drop(stashed_handle);

			if let Some(responder) = single_response_sender {
				let _ = responder.send_new(InternalPollResponse::WorkerTerminated);
			}

			let peer_requests = Publisher::new();
			worker.destroy().await;
			send_peer_requests(&peer_requests_sender, peer_requests.items()).await;
		});
	}
}

enum ResponseSender<Response> {
	Single(oneshot::Sender<InternalPollResponse<Response>>),
	Many(oneshot::Sender<InternalPollResponse<Vec<Response>>>),
}

impl<Response> ResponseSender<Response> {
	fn send_new(self, message: InternalPollResponse<Response>) -> std::result::Result<(), InternalPollResponse<Response>> {
		match self {
			Self::Single(sender) => sender.send(message),
			Self::Many(sender) => {
				let message = match message {
					InternalPollResponse::Ok(item) => InternalPollResponse::Ok(Vec::from([item])),
					InternalPollResponse::Ceeded => InternalPollResponse::Ceeded,
					InternalPollResponse::WorkerTerminated => InternalPollResponse::WorkerTerminated,
				};

				sender.send(message).map_err(|err| match err {
					InternalPollResponse::Ok(mut many) => InternalPollResponse::Ok(many.pop().unwrap()),
					InternalPollResponse::Ceeded => InternalPollResponse::Ceeded,
					InternalPollResponse::WorkerTerminated => InternalPollResponse::WorkerTerminated,
				})
			}
		}
	}
}
