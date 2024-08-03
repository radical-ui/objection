use std::future::{pending, Future};

use crate::worker::Worker;

pub enum SendResult<T> {
	Sent,
	Closed(T),
	Failed(T),
}

pub enum DropReason {
	Ceeded,
	WorkerTerminated,
	HandleClosed,
}

pub trait WorkerHandle<Request, Response> {
	/// Recieve the next request from the handle. A return value of None indicates that the handle has been closed.
	fn recv(&mut self) -> impl Future<Output = Option<Request>> + Send;
	/// Send a response to the handle.
	fn send(&mut self, response: Response) -> impl Future<Output = SendResult<Response>> + Send;
	/// Called right before the handle is going to be dropped. Contains a reason for why the handle will be dropped.
	fn will_drop(&mut self, reason: DropReason) -> impl Future<Output = ()> + Send;
}

pub struct NoopHandle;

impl<Request, Response> WorkerHandle<Request, Response> for NoopHandle
where
	Response: Send,
{
	async fn recv(&mut self) -> Option<Request> {
		None
	}

	async fn send(&mut self, message: Response) -> SendResult<Response> {
		SendResult::Closed(message)
	}

	async fn will_drop(&mut self, _: DropReason) {}
}

pub async fn recv_from_handle<W, H>(handle: Option<&mut H>) -> Option<W::Request>
where
	W: Worker,
	H: WorkerHandle<W::Request, W::Response>,
{
	if let Some(handle) = handle {
		handle.recv().await
	} else {
		pending().await
	}
}
