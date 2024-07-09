use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Component, HasActionKey};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum ModalSize {
	Small,
	#[default]
	Medium,
	Large,
}

/// A modal that appears over all existing content, using the context from where it is placed.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Modal {
	pub title: String,
	pub description: Option<String>,
	pub cancel_action_label: Option<String>,
	pub cancel_action: Option<ActionKey>,
	pub finish_action_label: Option<String>,
	pub finish_action: Option<ActionKey>,
	pub size: ModalSize,
	pub body: Option<Box<Component>>,
}

impl Modal {
	pub fn new(title: impl Into<String>) -> Modal {
		Modal {
			title: title.into(),
			description: None,
			cancel_action: None,
			cancel_action_label: None,
			finish_action: None,
			finish_action_label: None,
			body: None,
			size: Default::default(),
		}
	}

	pub fn description(mut self, description: impl Into<String>) -> Modal {
		self.description = Some(description.into());

		self
	}

	pub fn cancel_action(mut self, label: impl Into<String>, action: impl HasActionKey) -> Modal {
		self.cancel_action_label = Some(label.into());
		self.cancel_action = Some(action.get_action_key());

		self
	}

	pub fn finish_action(mut self, label: impl Into<String>, action: impl HasActionKey) -> Modal {
		self.finish_action_label = Some(label.into());
		self.finish_action = Some(action.get_action_key());

		self
	}

	pub fn size(mut self, size: ModalSize) -> Modal {
		self.size = size;

		self
	}

	pub fn body(mut self, body: impl Into<Component>) -> Modal {
		self.body = Some(Box::new(body.into()));

		self
	}
}
