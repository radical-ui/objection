use publisher::Publisher;
use serde_json::Value;
use uuid::Uuid;

use crate::session::DownstreamMessage;

#[derive(Debug, Clone, Copy)]
pub struct Controller<'a> {
	id: &'a Uuid,
	downstream_messages: &'a Publisher<DownstreamMessage>,
}

impl Controller<'_> {
	pub fn get_session_id(&self) -> &Uuid {
		&self.id
	}

	pub fn set_object(&self, id: impl Into<String>, data: Value) {
		self.downstream_messages.publish(DownstreamMessage::SetObject { id: id.into(), data });
	}

	pub fn remove_object(&self, id: impl Into<String>) {
		self.downstream_messages.publish(DownstreamMessage::RemoveObject { id: id.into() });
	}
}

pub fn new_controller<'a>(id: &'a Uuid, downstream_messages: &'a Publisher<DownstreamMessage>) -> Controller<'a> {
	Controller { id, downstream_messages }
}
