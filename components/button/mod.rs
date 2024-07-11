use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Color, HasActionKey, IconName};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum ButtonSize {
	Small,
	#[default]
	Medium,
	Large,
}

/// A button that has a label and an action.
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	Foo,
/// 	Bar,
/// }
///
/// Flex::new(FlexKind::Column)
/// 	.gap(10)
/// 	.align(FlexAlign::Center)
/// 	.justify(FlexJustify::Center)
/// 	.auto_item(
/// 		Flex::new(FlexKind::Row)
/// 			.gap(10)
/// 			.align(FlexAlign::Center)
/// 			.auto_item(
/// 				Button::new("Small Button")
/// 					.action(Event::Foo)
/// 					.size(ButtonSize::Small)
/// 			)
/// 			.auto_item(
/// 				Button::new("Medium Button")
/// 					.action(Event::Foo)
/// 			)
/// 			.auto_item(
/// 				Button::new("Large Button")
/// 					.action(Event::Bar)
/// 					.size(ButtonSize::Large)
/// 			)
/// 	)
/// 	.auto_item(
/// 		Flex::new(FlexKind::Row)
/// 			.gap(10)
/// 			.auto_item(
/// 				Button::new("Fore Button")
/// 					.action(Event::Foo)
/// 					.color(Color::Fore(5))
/// 			)
/// 			.auto_item(
/// 				Button::new("Success Button")
/// 					.action(Event::Foo)
/// 					.color(Color::Success(100))
/// 			)
/// 			.auto_item(
/// 				Button::new("Danger Button")
/// 					.action(Event::Foo)
/// 					.color(Color::Danger(100))
/// 			)
/// 	)
/// 	.auto_item(
/// 		Flex::new(FlexKind::Row)
/// 			.gap(10)
/// 			.auto_item(
/// 				Button::new("Leading Icon")
/// 					.action(Event::Foo)
/// 					.leading_icon("mdi-ab-testing")
/// 			)
/// 			.auto_item(
/// 				Button::new("Trailing Icon")
/// 					.action(Event::Foo)
/// 					.trailing_icon("mdi-ab-testing")
/// 			)
/// 			.auto_item(
/// 				Button::new("Both")
/// 					.action(Event::Bar)
/// 					.trailing_icon("mdi-ab-testing")
/// 					.leading_icon("mdi-ab-testing")
/// 					.outline()
/// 			)
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Button {
	pub label: String,
	pub action: Option<ActionKey>,
	pub color: Color,
	pub leading_icon: Option<IconName>,
	pub trailing_icon: Option<IconName>,
	pub size: ButtonSize,
	pub full: bool,
	pub outline: bool,
}

impl Button {
	pub fn new(label: impl Into<String>) -> Button {
		Button {
			label: label.into(),
			action: None,
			color: Color::Primary(100),
			leading_icon: None,
			trailing_icon: None,
			size: ButtonSize::default(),
			full: false,
			outline: false,
		}
	}

	pub fn action(mut self, action: impl HasActionKey) -> Button {
		self.action = Some(action.get_action_key());

		self
	}

	pub fn leading_icon(mut self, icon_name: impl Into<IconName>) -> Button {
		self.leading_icon = Some(icon_name.into());

		self
	}

	pub fn trailing_icon(mut self, icon_name: impl Into<IconName>) -> Button {
		self.trailing_icon = Some(icon_name.into());

		self
	}

	pub fn color(mut self, color: Color) -> Button {
		self.color = color;

		self
	}

	pub fn size(mut self, size: ButtonSize) -> Button {
		self.size = size;

		self
	}

	pub fn full(mut self) -> Button {
		self.full = true;

		self
	}

	pub fn outline(mut self) -> Button {
		self.outline = true;

		self
	}
}
