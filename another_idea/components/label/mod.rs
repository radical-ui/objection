use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Color, Component, HasActionKey};

/// A simple label
///
/// **Example**
///
/// ```rust
/// #[derive(Serialize, Deserialize, HasActionKey)]
/// enum Event {
/// 	Foo
/// }
///
/// Flex::new(FlexKind::Column)
/// 	.gap(5)
/// 	.justify(FlexJustify::Center)
/// 	.align(FlexAlign::Center)
/// 	.auto_item(Label::new("Some Label"))
/// 	.auto_item(Label::new("Italic").italic())
/// 	.auto_item(Label::new("Bold").bold())
/// 	.auto_item(Label::new("Another Color").color(Color::Primary(100)))
/// 	.auto_item(Label::new("This one is editable").edit_action(Event::Foo).color(Color::Primary(100)))
/// 	.auto_item(
/// 		Flex::new(FlexKind::Row)
/// 			.auto_item(Label::new("And so is this").edit_action(Event::Foo))
/// 			.auto_item(Label::new("And this too (with a placeholder)").edit_action(Event::Foo).placeholder("This is the placeholder!!!! It is pretty long."))
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Label {
	pub text: String,
	pub is_bold: bool,
	pub is_italic: bool,
	pub color: Color,
	pub edit_action: Option<ActionKey>,
	pub placeholder: Option<String>,
}

impl Label {
	pub fn new(text: impl Into<String>) -> Label {
		Label {
			text: text.into(),
			is_bold: false,
			is_italic: false,
			color: Color::Fore(80),
			edit_action: None,
			placeholder: None,
		}
	}

	pub fn bold(mut self) -> Label {
		self.is_bold = true;

		self
	}

	pub fn italic(mut self) -> Label {
		self.is_italic = true;

		self
	}

	pub fn color(mut self, color: Color) -> Label {
		self.color = color;

		self
	}

	pub fn edit_action(mut self, action: impl HasActionKey) -> Label {
		self.edit_action = Some(action.get_action_key());

		self
	}

	pub fn placeholder(mut self, placeholder: impl Into<String>) -> Label {
		self.placeholder = Some(placeholder.into());

		self
	}
}

impl From<&str> for Label {
	fn from(value: &str) -> Self {
		Label::new(value)
	}
}

impl From<String> for Label {
	fn from(value: String) -> Self {
		Label::new(value)
	}
}

impl From<&str> for Component {
	fn from(value: &str) -> Self {
		Label::new(value).into()
	}
}

impl From<String> for Component {
	fn from(value: String) -> Self {
		Label::new(value).into()
	}
}
