use async_worker::PeerRequest;
use publisher::Publisher;
use serde_json::Value;
use uuid::Uuid;

use crate::session::{DownstreamMessage, SessionEvent};

#[derive(Debug)]
pub struct Controller<'a, PeerEvent> {
	id: &'a Uuid,
	downstream_messages: &'a Publisher<DownstreamMessage>,
	peer_requests: &'a Publisher<PeerRequest<Uuid, SessionEvent<PeerEvent>>>,
}

impl<PeerEvent> Clone for Controller<'_, PeerEvent> {
	fn clone(&self) -> Self {
		Controller {
			id: self.id,
			downstream_messages: self.downstream_messages,
			peer_requests: self.peer_requests,
		}
	}
}

impl<PeerEvent> Copy for Controller<'_, PeerEvent> {}

impl<PeerEvent> Controller<'_, PeerEvent> {
	pub fn get_session_id(&self) -> &Uuid {
		&self.id
	}

	pub fn set_object(&self, id: impl Into<String>, data: Value) {
		self.downstream_messages.publish(DownstreamMessage::SetObject { id: id.into(), data });
	}

	pub fn remove_object(&self, id: impl Into<String>) {
		self.downstream_messages.publish(DownstreamMessage::RemoveObject { id: id.into() });
	}

	pub fn send_peer_event(&self, id: Uuid, event: PeerEvent) {
		self.peer_requests.publish(PeerRequest {
			id: id.into(),
			content: SessionEvent::PeerEvent(event),
		})
	}
}

pub fn new_controller<'a, PeerEvent>(
	id: &'a Uuid,
	downstream_messages: &'a Publisher<DownstreamMessage>,
	peer_requests: &'a Publisher<PeerRequest<Uuid, SessionEvent<PeerEvent>>>,
) -> Controller<'a, PeerEvent> {
	Controller {
		id,
		downstream_messages,
		peer_requests,
	}
}
