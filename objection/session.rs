use anyhow::Result;
use async_worker::Worker;
use log::warn;
use publisher::Publisher;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use uuid::Uuid;

use crate::{controller::new_controller, Controller};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UpstreamMessage {
	Watch { request_id: Uuid, id: String },
	Unwatch { request_id: Uuid, id: String },
	EmitBindingUpdate { request_id: Uuid, key: String, data: Value },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DownstreamMessage {
	RemoveObject {
		id: String,
	},
	SetObject {
		id: String,
		data: Value,
	},
	Acknowledge {
		request_id: Option<Uuid>,
		error: Option<String>,
		retry_after_seconds: Option<u32>,
	},
}

pub enum SessionEvent<T> {
	ClientMessage(UpstreamMessage),
	PeerEvent(T),
	Init { auth_token: Option<String>, path: String },
}

#[derive(Debug)]
pub struct SessionWorkerContext<S>
where
	S: Session,
{
	pub session_context: S::Context,
}

impl<S> Clone for SessionWorkerContext<S>
where
	S: Session,
{
	fn clone(&self) -> Self {
		SessionWorkerContext {
			session_context: self.session_context.clone(),
		}
	}
}

pub struct SessionWorker<S>
where
	S: Session,
{
	id: Uuid,
	session: Option<S>,
	session_context: S::Context,
}

impl<S> Worker for SessionWorker<S>
where
	S: Session + 'static + Send,
{
	type Context = SessionWorkerContext<S>;
	type Id = Uuid;
	type Request = SessionEvent<S::PeerEvent>;
	type Response = Vec<DownstreamMessage>;

	async fn create(id: &Self::Id, context: Self::Context) -> Self {
		SessionWorker {
			id: id.clone(),
			session: None,
			session_context: context.session_context,
		}
	}

	async fn handle(&mut self, request: Self::Request) -> Self::Response {
		let session = match &mut self.session {
			Some(session) => session,
			None => {
				warn!("BUG: Session could not be constructed and handle was called again");
				return Vec::new();
			}
		};

		let publisher = Publisher::new();
		let controller = new_controller(&self.id, &publisher);

		match request {
			SessionEvent::ClientMessage(message) => {
				let (request_id, result) = match message {
					UpstreamMessage::Watch { request_id, id } => (request_id, session.watch_object(&id, &self.session_context, controller).await),
					UpstreamMessage::Unwatch { request_id, id } => (request_id, session.unwatch_object(&id, &self.session_context, controller).await),
					UpstreamMessage::EmitBindingUpdate { request_id, key, data } => {
						(request_id, session.update_binding(&key, data, &self.session_context, controller).await)
					}
				};

				if let Err(error) = result {
					warn!("error in handler: {error:?}");

					publisher.publish(DownstreamMessage::Acknowledge {
						request_id: None,
						error: Some(error.to_string()),
						retry_after_seconds: None,
					});
				} else {
					publisher.publish(DownstreamMessage::Acknowledge {
						request_id: Some(request_id),
						error: None,
						retry_after_seconds: None,
					});
				}
			}
			SessionEvent::PeerEvent(_) => todo!(),
			SessionEvent::Init { auth_token, path } => {
				match S::create(auth_token, path, &self.session_context, controller).await {
					Err(error) => {
						publisher.publish(DownstreamMessage::Acknowledge {
							request_id: None,
							error: Some(error.to_string()),
							retry_after_seconds: None,
						});
					}

					Ok(session) => {
						self.session = Some(session);
					}
				};
			}
		}

		publisher.items()
	}

	async fn destroy(self) {
		if let Some(session) = self.session {
			session.destroy().await;
		}
	}
}

pub trait Session
where
	Self: Sized,
{
	type Context: 'static + Clone + Send + Sync;
	type PeerEvent: 'static + Clone + Send + Sync;

	fn create(
		auth_token: Option<String>,
		path: String,
		context: &Self::Context,
		controller: Controller<'_>,
	) -> impl Future<Output = Result<Self>> + Send + Sync;

	fn watch_object(&mut self, id: &str, context: &Self::Context, controller: Controller<'_>) -> impl Future<Output = Result<()>> + Send + Sync;

	#[allow(unused_variables)]
	fn unwatch_object(&mut self, id: &str, context: &Self::Context, controller: Controller<'_>) -> impl Future<Output = Result<()>> + Send + Sync {
		async { Ok(()) }
	}

	#[allow(unused_variables)]
	fn update_binding(
		&mut self,
		key: &str,
		data: Value,
		context: &Self::Context,
		controller: Controller<'_>,
	) -> impl Future<Output = Result<()>> + Send + Sync {
		async { Ok(()) }
	}

	#[allow(unused_variables)]
	fn provide_auth_token(&mut self, token: String) -> impl Future<Output = ()> + Send + Sync {
		async {}
	}

	#[allow(unused_variables)]
	fn handle_peer_event(&mut self, event: Self::PeerEvent) -> impl Future<Output = ()> + Send + Sync {
		async {}
	}

	fn destroy(self) -> impl Future<Output = ()> + Send + Sync {
		async {}
	}
}
