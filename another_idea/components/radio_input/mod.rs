use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Component, HasActionKey};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RadioItem {
	id: usize,
	title: String,
	description: Option<Component>,
}

/// TODO
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	Batter,
/// }
///
/// Flex::new(FlexKind::Column)
/// 	.gap(30)
/// 	.auto_item(
/// 		RadioInput::new()
/// 			.action(Event::Batter)
/// 			.item(0, "Red")
/// 			.item(1, "Green")
/// 	)
/// 	.auto_item(
/// 		RadioInput::new()
/// 			.action(Event::Batter)
/// 			.item(0, "Hi")
/// 			.described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend"))
/// 			.described_item(2, "Adieu", Label::new("The french form of \"Bye\""))
/// 	)
/// 	.auto_item(
/// 		RadioInput::new()
/// 			.item(0, "all are disabled here")
/// 			.described_item(1, "Bye", Label::new("This is greeting that people say when they are bidding farewell to a friend"))
/// 			.described_item(2, "Adieu", Label::new("The french form of \"Bye\""))
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RadioInput {
	pub action: Option<ActionKey>,
	pub items: Vec<RadioItem>,
	pub initial_value: Option<usize>,
}

impl RadioInput {
	pub fn new() -> RadioInput {
		RadioInput {
			items: Vec::new(),
			action: None,
			initial_value: None,
		}
	}

	pub fn action(mut self, action: impl HasActionKey) -> RadioInput {
		self.action = Some(action.get_action_key());

		self
	}

	pub fn item(mut self, id: usize, title: impl Into<String>) -> RadioInput {
		self.items.push(RadioItem {
			id,
			title: title.into(),
			description: None,
		});

		self
	}

	pub fn described_item(mut self, id: usize, title: impl Into<String>, description: impl Into<Component>) -> RadioInput {
		self.items.push(RadioItem {
			id,
			title: title.into(),
			description: Some(description.into()),
		});

		self
	}

	pub fn initial_value(mut self, value: usize) -> RadioInput {
		self.initial_value = Some(value);

		self
	}
}
