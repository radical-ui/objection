use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum NestedFlowItem {
	Header { text: String },
	Content { header_text: String, content: Component },
}

/// TODO
///
/// **Indented Example**
///
/// ```rust
/// NestedFlow::new()
/// 	.indent()
/// 	.header("Created Alone")
/// 	.content("Options", Label::new("Hi there!"))
/// 	.content("Evil Plans", Label::new("Bad bad here"))
/// 	.content("Good Plans", Label::new("Good good here!"))
/// ```
/// 
/// **Not Indented Example**
///
/// ```rust
/// NestedFlow::new()
/// 	.header("Created Alone")
/// 	.content("Options", Label::new("Hi there!"))
/// 	.content("Evil Plans", Label::new("Bad bad here"))
/// 	.content("Good Plans", Label::new("Good good here!"))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NestedFlow {
	indent: bool,
	items: Vec<NestedFlowItem>,
}

impl NestedFlow {
	pub fn new() -> NestedFlow {
		NestedFlow {
			indent: false,
			items: Vec::new(),
		}
	}

	pub fn indent(mut self) -> NestedFlow {
		self.indent = true;

		self
	}

	pub fn header(mut self, text: impl Into<String>) -> NestedFlow {
		self.items.push(NestedFlowItem::Header { text: text.into() });

		self
	}

	pub fn content(mut self, header: impl Into<String>, component: impl Into<Component>) -> NestedFlow {
		self.items.push(NestedFlowItem::Content {
			header_text: header.into(),
			content: component.into(),
		});

		self
	}
}
