use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Component, HasActionKey};

/// TODO
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	Foo,
/// 	Bar,
/// 	Bin,
/// }
///
/// Breadcrumbs::new()
/// 	.crumb(Event::Foo, "Hi")
/// 	.crumb(Event::Bar, "Bye")
/// 	.crumb(Event::Bin, "Bock")
/// 	.current("This")
/// 	.body("Some Body")
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Breadcrumbs {
	pub crumbs: Vec<(ActionKey, String)>,
	pub current: Option<String>,
	pub body: Option<Box<Component>>,
}

impl Breadcrumbs {
	pub fn new() -> Breadcrumbs {
		Breadcrumbs {
			crumbs: Vec::new(),
			current: None,
			body: None,
		}
	}

	pub fn crumb(mut self, key: impl HasActionKey, text: impl Into<String>) -> Breadcrumbs {
		self.crumbs.push((key.get_action_key(), text.into()));

		self
	}

	pub fn current(mut self, text: impl Into<String>) -> Breadcrumbs {
		self.current = Some(text.into());

		self
	}

	pub fn body(mut self, body: impl Into<Component>) -> Breadcrumbs {
		self.body = Some(Box::new(body.into()));

		self
	}
}
