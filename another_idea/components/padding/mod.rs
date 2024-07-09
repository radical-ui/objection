use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// A container with padding.
///
/// **Example**
///
/// ```rust
/// Padding::all(30).body(Card::new().body(Label::new("See, it is padded!")))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Padding {
	pub top: usize,
	pub left: usize,
	pub bottom: usize,
	pub right: usize,
	pub body: Option<Box<Component>>,
}

impl Padding {
	pub fn all(amount: usize) -> Padding {
		Padding {
			top: amount,
			left: amount,
			right: amount,
			bottom: amount,
			body: None,
		}
	}

	pub fn each(top: usize, left: usize, bottom: usize, right: usize) -> Padding {
		Padding {
			top,
			left,
			bottom,
			right,
			body: None,
		}
	}

	pub fn horizontal(mut self, amount: usize) -> Padding {
		self.left = amount;
		self.right = amount;

		self
	}

	pub fn vertical(mut self, amount: usize) -> Padding {
		self.top = amount;
		self.bottom = amount;

		self
	}

	pub fn body(mut self, child: impl Into<Component>) -> Padding {
		self.body = Some(Box::new(child.into()));

		self
	}
}
