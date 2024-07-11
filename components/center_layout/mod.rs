use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// TODO
///
/// **Example**
///
/// ```rust
/// CenterLayout::new("Normal Center Layout").subtitle("Some Subtitle").body(Button::new("Hello there!").full())
/// ```
///
/// ```rust
/// CenterLayout::new("Thin Center Layout").subtitle("Some Subtitle").thin().body(Button::new("Hello there!").full())
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CenterLayout {
	pub title: String,
	pub subtitle: Option<String>,
	pub body: Option<Box<Component>>,
	pub thin: bool,
}

impl CenterLayout {
	pub fn new(title: impl Into<String>) -> CenterLayout {
		CenterLayout {
			title: title.into(),
			subtitle: None,
			body: None,
			thin: false,
		}
	}

	pub fn subtitle(mut self, subtitle: impl Into<String>) -> CenterLayout {
		self.subtitle = Some(subtitle.into());

		self
	}

	pub fn body(mut self, body: impl Into<Component>) -> CenterLayout {
		self.body = Some(Box::new(body.into()));

		self
	}

	pub fn thin(mut self) -> CenterLayout {
		self.thin = true;

		self
	}
}
