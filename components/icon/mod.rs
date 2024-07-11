use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Color;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IconName(String);

impl From<&str> for IconName {
	fn from(value: &str) -> Self {
		IconName(value.into())
	}
}

impl From<String> for IconName {
	fn from(value: String) -> Self {
		IconName(value)
	}
}

/// TODO
///
/// **Example**
///
/// ```rust
/// Flex::new(FlexKind::Row)
/// 	.gap(30)
/// 	.justify(FlexJustify::Center)
/// 	.align(FlexAlign::Center)
/// 	.auto_item( Icon::new("mdi-ab-testing", 30).color(Color::Primary(100)))
/// 	.auto_item( Icon::new("mdi-account-arrow-left", 30).color(Color::Success(100)))
/// 	.auto_item( Icon::new("mdi-access-point", 30).color(Color::Danger(50)))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Icon {
	pub name: IconName,
	pub title: Option<String>,
	pub size: usize,
	pub color: Color,
}

impl Icon {
	pub fn new(name: impl Into<IconName>, size: usize) -> Icon {
		Icon {
			name: name.into(),
			title: None,
			size,
			color: Color::Fore(100),
		}
	}

	pub fn title(mut self, title: impl Into<String>) -> Icon {
		self.title = Some(title.into());

		self
	}

	pub fn color(mut self, color: Color) -> Icon {
		self.color = color;

		self
	}

	pub fn size(mut self, size: usize) -> Icon {
		self.size = size;

		self
	}
}
