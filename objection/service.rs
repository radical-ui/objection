use anyhow::Result;
use async_worker::{Queue, QueueBuilder, SendResult, WorkerHandle};
use bytes::Bytes;
use fastwebsockets::{upgrade::upgrade, Frame, OpCode, Payload, WebSocket, WebSocketError};
use futures::future::{ready, Ready};
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use hyper::{body::Body, upgrade::Upgraded};
use hyper_util::rt::TokioIo;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};
use std::{
	convert::Infallible,
	fmt::Display,
	future::Future,
	marker::PhantomData,
	sync::Arc,
	task::{Context, Poll},
};
use tower::Service;
use uuid::Uuid;

use crate::{
	session::{IncomingSocketMessage, OutgoingSocketMessage, Session, SessionEvent, SessionWorker, SessionWorkerContext},
	ObjectRouter,
};

pub struct ObjectionService<S>
where
	S: Session + Send + 'static,
{
	queue: Arc<Queue<SessionWorker<S>, SocketHandle<S::PeerEvent>>>,
}

impl<S> ObjectionService<S>
where
	S: Session + Send + 'static,
{
	pub fn new(router: ObjectRouter<S>, context: S::Context) -> ObjectionService<S> {
		ObjectionService {
			queue: Arc::new(QueueBuilder::default().build(SessionWorkerContext {
				shared_router: Arc::new(router),
				session_context: context,
			})),
		}
	}

	fn call_internal<Body>(&mut self, mut request: Request<Body>) -> Result<Response<Full<Bytes>>, Infallible> {
		#[derive(Debug, Serialize, Deserialize)]
		struct QueryParams {
			auth_token: Option<String>,
			session_id: Uuid,
		}

		let (empty_response, fut) = match upgrade(&mut request) {
			Ok(inner) => inner,
			Err(_) => return Ok(bad_response("Failed to upgrade request")),
		};

		let query_string = match request.uri().query() {
			Some(query) => query,
			None => {
				warn!("recieved no query parameters in request");

				return Ok(bad_response("Expected to recieve a querystring with the request"));
			}
		};

		let query_params = match serde_qs::from_str::<QueryParams>(query_string) {
			Ok(params) => params,
			Err(error) => {
				warn!("recieved invalid query parameters in request: {error:?}");

				return Ok(bad_response("Recieved invalid query parameters: {error}"));
			}
		};

		let queue = self.queue.clone();

		tokio::spawn(async move {
			match fut.await {
				Ok(socket) => {
					let mut handle = SocketHandle {
						socket,
						_phantom_data: PhantomData,
					};

					let enqueue_result = queue
						.enqueue(
							&query_params.session_id,
							SessionEvent::Init {
								auth_token: query_params.auth_token,
							},
						)
						.await;

					match enqueue_result {
						Ok(_) => (),
						Err(async_worker::Error::WorkerAtCapacity) => {
							warn!(
								"the worker for session '{}', which just connected, was already at capacity",
								query_params.session_id
							);

							handle.send_init_error("You've been rate-limited", Some(10)).await;
						}
						Err(error) => {
							error!("failed to enqueue the init message: {error}");

							handle.send_init_error("We've encountered an internal error.", Some(10)).await;
						}
					}

					if let Err(error) = queue.register_handle(&query_params.session_id, handle) {
						error!("failed to send socket handle to worker: {error}");
					}
				}
				Err(error) => {
					error!("Failed to convert websocket upgrade into websocket: {error:?}")
				}
			}
		});

		Ok(empty_response.map(|_| Full::new(Bytes::new())))
	}
}

impl<ReqBody, S> Service<Request<ReqBody>> for ObjectionService<S>
where
	S: Session + Send + 'static,
{
	type Response = Response<Full<Bytes>>;
	type Error = Infallible;
	type Future = Ready<Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		std::task::Poll::Ready(Ok(()))
	}

	fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
		ready(self.call_internal(request))
	}
}

impl<S> Clone for ObjectionService<S>
where
	S: Session + Send,
{
	fn clone(&self) -> Self {
		ObjectionService { queue: self.queue.clone() }
	}
}

fn bad_response(message: impl Display) -> Response<Full<Bytes>> {
	let mut response = Response::new(Full::new(Bytes::from(message.to_string())));
	*response.status_mut() = StatusCode::BAD_REQUEST;

	response
}

struct SocketHandle<PeerEvent> {
	socket: WebSocket<TokioIo<Upgraded>>,
	_phantom_data: PhantomData<PeerEvent>,
}

impl<PeerEvent> SocketHandle<PeerEvent>
where
	PeerEvent: Send,
{
	async fn send_init_error(&mut self, message: &str, retry_after_seconds: Option<u32>) {
		let result = self
			.send(Vec::from([OutgoingSocketMessage::Acknowledge {
				request_id: None,
				error: Some(message.into()),
				retry_after_seconds,
			}]))
			.await;

		if let SendResult::Failed(_) = result {
			error!("failed to beam an error to the client")
		}
	}
}

impl<PeerEvent> WorkerHandle<SessionEvent<PeerEvent>, Vec<OutgoingSocketMessage>> for SocketHandle<PeerEvent>
where
	Self: Send,
{
	async fn recv(&mut self) -> Option<SessionEvent<PeerEvent>> {
		loop {
			let frame_res = self.socket.read_frame().await;
			let frame = match frame_res {
				Ok(frame) => frame,
				Err(WebSocketError::ConnectionClosed) => break None,
				Err(error) => {
					error!("Failed to read frame from socket. Closing to prevent infinite loop: {error}");
					break None;
				}
			};

			match frame.opcode {
				OpCode::Text => (),
				code => {
					warn!("Recieved non-text frame ({code:?}) from socket. Skipping to next message.");

					continue;
				}
			};

			let message = match from_slice::<IncomingSocketMessage>(&frame.payload) {
				Ok(message) => message,
				Err(error) => {
					warn!("Failed to deserialize message from socket. Skipping to next message: {error}");
					self.send(Vec::from([OutgoingSocketMessage::Acknowledge {
						request_id: None,
						error: Some("Message could not be deserialized".into()),
						retry_after_seconds: None,
					}]))
					.await;

					continue;
				}
			};

			break Some(SessionEvent::ClientMessage(message));
		}
	}

	async fn send(&mut self, response: Vec<OutgoingSocketMessage>) -> SendResult<Vec<OutgoingSocketMessage>> {
		let result = self
			.socket
			.write_frame(Frame::text(Payload::Owned(to_string(&response).unwrap().into_bytes())))
			.await;

		match result {
			Ok(_) => SendResult::Sent,
			Err(WebSocketError::ConnectionClosed) => SendResult::Closed(response),
			Err(error) => {
				warn!("failed to send message to client: {error}");

				SendResult::Failed(response)
			}
		}
	}

	fn will_drop(&mut self, _: async_worker::DropReason) -> impl Future<Output = ()> + Send {
		async {}
	}
}
