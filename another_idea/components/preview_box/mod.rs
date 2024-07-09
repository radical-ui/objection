use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// A ui-decorated box for displaying content
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PreviewBox {
	pub title: String,
	pub child: Box<Component>,
}

impl PreviewBox {
	pub fn new(title: impl Into<String>, child: impl Into<Component>) -> PreviewBox {
		PreviewBox {
			title: title.into(),
			child: Box::new(child.into()),
		}
	}
}
