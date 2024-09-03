use async_worker::Worker;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, iter::once, sync::Arc};
use uuid::Uuid;

use crate::{
	object::Object,
	router::{ObjectRouter, Resolution, ResolverAction, RouteResolver},
	theme::Theme,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "def", rename_all = "snake_case")]
pub enum IncomingSocketMessage {
	Watch { request_id: Uuid, id: String },
	Unwatch { request_id: Uuid, id: String },
	PerformOperation { request_id: Uuid, object_id: String, key: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "def", rename_all = "snake_case")]
pub enum OutgoingSocketMessage {
	Init {
		theme: Theme,
		objects: HashMap<String, Object>,
	},
	RemoveObject {
		id: String,
	},
	SetObject {
		id: String,
		object: Object,
	},
	SetTheme {
		theme: Theme,
	},
	Acknowledge {
		request_id: Option<Uuid>,
		error: Option<String>,
		retry_after_seconds: Option<u32>,
	},
}

pub enum SessionEvent<T> {
	ClientMessage(IncomingSocketMessage),
	PeerEvent(T),
	Init { auth_token: Option<String> },
}

#[derive(Debug)]
pub struct SessionWorkerContext<S>
where
	S: Session,
{
	pub shared_router: Arc<ObjectRouter<S>>,
	pub session_context: S::Context,
}

impl<S> Clone for SessionWorkerContext<S>
where
	S: Session,
{
	fn clone(&self) -> Self {
		SessionWorkerContext {
			shared_router: self.shared_router.clone(),
			session_context: self.session_context.clone(),
		}
	}
}

pub struct SessionWorker<S>
where
	S: Session,
{
	resolver: RouteResolver<S>,
	session: S,
}

impl<S> Worker for SessionWorker<S>
where
	S: Session + 'static + Send,
{
	type Context = SessionWorkerContext<S>;
	type Id = Uuid;
	type Request = SessionEvent<S::PeerEvent>;
	type Response = Vec<OutgoingSocketMessage>;

	async fn create(id: &Self::Id, context: Self::Context) -> Self {
		SessionWorker {
			resolver: RouteResolver::from_shared_router(context.shared_router),
			session: S::create(id, context.session_context).await,
		}
	}

	async fn handle(&mut self, request: Self::Request) -> Self::Response {
		match request {
			SessionEvent::ClientMessage(message) => {
				let mut resolution = Resolution::empty();

				let (request_id, result) = match message {
					IncomingSocketMessage::Watch { request_id, id } => (request_id, self.resolver.watch(&id, &mut self.session, &mut resolution).await),
					IncomingSocketMessage::Unwatch { request_id, id } => (request_id, Ok(self.resolver.unwatch(&id).await)),
					IncomingSocketMessage::PerformOperation { request_id, object_id, key } => {
						(request_id, self.resolver.perform_operation(&object_id, &key, &mut resolution).await)
					}
				};

				if let Err(error) = result {
					warn!("error in handler: {error:?}");

					Vec::from([OutgoingSocketMessage::Acknowledge {
						request_id: None,
						error: Some(error.to_string()),
						retry_after_seconds: Some(5),
					}])
				} else {
					resolution
						.actions
						.drain(..)
						.map(|action| match action {
							ResolverAction::RemoveObject { id } => OutgoingSocketMessage::RemoveObject { id },
							ResolverAction::SetObject { id, object } => OutgoingSocketMessage::SetObject { id, object },
						})
						.chain(once(OutgoingSocketMessage::Acknowledge {
							request_id: Some(request_id),
							error: None,
							retry_after_seconds: None,
						}))
						.collect()
				}
			}
			SessionEvent::PeerEvent(_) => todo!(),
			SessionEvent::Init { auth_token } => {
				if let Some(token) = auth_token {
					self.session.provide_auth_token(token).await;
				}

				let theme = self.session.get_theme().await;
				let mut objects = HashMap::new();

				for entry_id in theme.get_entry_object_ids() {
					let mut resolution = Resolution::empty();

					if let Err(error) = self.resolver.watch(&entry_id, &mut self.session, &mut resolution).await {
						error!("resolver.watch for an entry id '{entry_id}' should not error, but it did: {error:?}");

						continue;
					}

					for action in resolution.actions {
						match action {
							ResolverAction::RemoveObject { id } => {
								warn!("resolver.watch for an entry id should not be removing objects, but it did try to remove '{id}'")
							}
							ResolverAction::SetObject { id, object } => {
								objects.insert(id, object);
							}
						}
					}
				}

				Vec::from([OutgoingSocketMessage::Init { theme, objects }])
			}
		}
	}

	fn destroy(self) -> impl Future<Output = ()> + Send {
		self.session.destroy()
	}
}

pub trait Session
where
	Self: Sized,
{
	type Context: 'static + Clone + Send + Sync;
	type PeerEvent: 'static + Clone + Send + Sync;

	fn create(id: &Uuid, context: Self::Context) -> impl Future<Output = Self> + Send + Sync;

	#[allow(unused_variables)]
	fn provide_auth_token(&mut self, token: String) -> impl Future<Output = ()> + Send + Sync {
		async {}
	}

	fn get_theme(&mut self) -> impl Future<Output = Theme> + Send + Sync;

	#[allow(unused_variables)]
	fn handle_peer_event(&mut self, event: Self::PeerEvent) -> impl Future<Output = ()> + Send + Sync {
		async {}
	}

	fn destroy(self) -> impl Future<Output = ()> + 'static + Send + Sync {
		async {}
	}
}
