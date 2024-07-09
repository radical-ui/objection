use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ActionKey, Component, HasActionKey};

/// A container that prefixes all actions triggered within with `scope`
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
///	enum Event {
/// 	Foo,
/// 	Bar,
/// }
///
/// ActionScope::new(Event::Foo).payload(serde_json::json!({ "here": true })).body(Button::new("Click me").action(Event::Bar))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ActionScope {
	pub scope: ActionKey,
	pub payload: Option<Value>,
	pub body: Option<Box<Component>>,
}

impl ActionScope {
	pub fn new(scope: impl HasActionKey) -> ActionScope {
		ActionScope {
			scope: scope.get_action_key(),
			payload: None,
			body: None,
		}
	}

	pub fn body(mut self, component: impl Into<Component>) -> ActionScope {
		self.body = Some(Box::new(component.into()));

		self
	}

	pub fn payload(mut self, value: Value) -> ActionScope {
		self.payload = Some(value);

		self
	}
}
