use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, HasActionKey, IconName};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeaderActionItem {
	pub action: ActionKey,
	pub icon: IconName,
	pub label: String,
}

impl HeaderActionItem {
	pub fn new(action: impl HasActionKey, icon: impl Into<IconName>, label: impl Into<String>) -> HeaderActionItem {
		HeaderActionItem {
			action: action.get_action_key(),
			icon: icon.into(),
			label: label.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum HeaderSize {
	Large,
	Medium,
	Small,
}

/// A simple page layout, with a title, subtitle, some possible action items, and a body. Additionally, a logo
/// can appear off to the right.
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// pub enum Event {
/// 	Foo,
/// 	Bar,
/// }
///
/// Flex::new(FlexKind::Column)
/// 	.gap(30)
/// 	.auto_item(
/// 		Header::new("With Action Items")
/// 			.subtitle("A subtitle here")
/// 			.size(HeaderSize::Large)
/// 			.action_item(Event::Foo, "mdi-pencil", "Do Foo")
/// 			.action_item(Event::Bar, "mdi-ab-testing", "A very long comment that will take up some notable space")
/// 	)
/// 	.auto_item(
/// 		Header::new("With Action Items")
/// 			.subtitle("A subtitle here")
/// 			.size(HeaderSize::Medium)
/// 			.action_item(Event::Foo, "mdi-pencil", "Do Foo")
/// 			.action_item(Event::Bar, "mdi-ab-testing", "Do Bar")
/// 	)
/// 	.auto_item(
/// 		Header::new("With Action Items")
/// 			.subtitle("A subtitle here")
/// 			.title_edit_action(Event::Foo)
/// 			.subtitle_edit_action(Event::Bar)
/// 			.subtitle_placeholder("No description")
/// 			.size(HeaderSize::Small)
/// 			.action_item(Event::Foo, "mdi-pencil", "Do Foo")
/// 			.action_item(Event::Bar, "mdi-ab-testing", "Do Bar")
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Header {
	pub title: String,
	pub title_edit_action: Option<ActionKey>,
	pub title_placeholder: Option<String>,
	pub subtitle: Option<String>,
	pub subtitle_edit_action: Option<ActionKey>,
	pub subtitle_placeholder: Option<String>,
	pub action_items: Vec<HeaderActionItem>,
	pub size: HeaderSize,
}

impl Header {
	pub fn new(title: impl Into<String>) -> Header {
		Header {
			title: title.into(),
			title_edit_action: None,
			title_placeholder: None,
			subtitle: None,
			subtitle_edit_action: None,
			subtitle_placeholder: None,
			action_items: Vec::new(),
			size: HeaderSize::Medium,
		}
	}

	pub fn title_edit_action(mut self, action: impl HasActionKey) -> Header {
		self.title_edit_action = Some(action.get_action_key());

		self
	}

	pub fn title_placeholder(mut self, placeholder: impl Into<String>) -> Header {
		self.title_placeholder = Some(placeholder.into());

		self
	}

	pub fn action_item(mut self, action: impl HasActionKey, icon: impl Into<IconName>, label: impl Into<String>) -> Header {
		self.action_items.push(HeaderActionItem::new(action, icon, label));

		self
	}

	pub fn subtitle(mut self, label: impl Into<String>) -> Header {
		self.subtitle = Some(label.into());

		self
	}

	pub fn subtitle_edit_action(mut self, action: impl HasActionKey) -> Header {
		self.subtitle_edit_action = Some(action.get_action_key());

		self
	}

	pub fn subtitle_placeholder(mut self, placeholder: impl Into<String>) -> Header {
		self.subtitle_placeholder = Some(placeholder.into());

		self
	}

	pub fn size(mut self, size: HeaderSize) -> Header {
		self.size = size;

		self
	}
}
