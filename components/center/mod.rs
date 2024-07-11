use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// TODO
///
/// **Example**
///
/// ```rust
/// Center::new().body(Label::new("Hello, World!"))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Center {
	pub body: Option<Box<Component>>,
}

impl Center {
	pub fn new() -> Center {
		Center { body: None }
	}

	pub fn body(mut self, body: impl Into<Component>) -> Center {
		self.body = Some(Box::new(body.into()));

		self
	}
}
