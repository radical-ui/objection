use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// TODO
///
/// **Example**
///
/// ```rust
/// #[derive(Debug, HasActionKey, Serialize, Deserialize)]
/// pub enum Event {
/// 	Foo
/// }
///
/// ActionBlocker::new().body(Button::new("Disabled").action(Event::Foo))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionBlocker {
	body: Option<Box<Component>>,
	block: bool,
}

impl ActionBlocker {
	pub fn new() -> ActionBlocker {
		ActionBlocker { body: None, block: true }
	}

	pub fn body(mut self, body: impl Into<Component>) -> ActionBlocker {
		self.body = Some(Box::new(body.into()));

		self
	}

	pub fn block_if(mut self, block: bool) -> ActionBlocker {
		self.block = block;

		self
	}
}
