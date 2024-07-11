use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Label;

/// TODO
///
/// **Example**
///
/// ```rust
/// CircleProgress::new()
/// 	.value(0.5)
/// 	.label("Hello")
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CircleProgress {
	size: usize,
	value: f64,
	label: Label,
}

impl CircleProgress {
	pub fn new() -> CircleProgress {
		CircleProgress {
			size: 150,
			value: 0.0,
			label: Label::new(""),
		}
	}

	pub fn size(mut self, size: usize) -> CircleProgress {
		self.size = size;

		self
	}

	pub fn value(mut self, value: f64) -> CircleProgress {
		self.value = value;

		self
	}

	pub fn label(mut self, label: impl Into<Label>) -> CircleProgress {
		self.label = label.into();

		self
	}
}
