use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Color, HasActionKey, IconName};

/// TODO
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	Foo
/// }
///
/// Flex::new(FlexKind::Row)
/// 	.gap(20)
/// 	.auto_item(
/// 		IconButton::new("mdi-ab-testing")
/// 			.color(Color::Primary(100))
/// 			.title("A description of what this does and it is a rather long description")
/// 			.size(40)
/// 			.action(Event::Foo)
/// 	)
/// 	.auto_item(IconButton::new("mdi-ab-testing"))
/// 	.auto_item(
/// 		IconButton::new("mdi-ab-testing")
/// 			.color(Color::Primary(100))
/// 			.action(Event::Foo)
/// 	)
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IconButton {
	pub name: IconName,
	pub title: Option<String>,
	pub action: Option<ActionKey>,
	pub size: usize,
	pub color: Color,
}

impl IconButton {
	pub fn new(icon_name: impl Into<IconName>) -> IconButton {
		IconButton {
			name: icon_name.into(),
			title: None,
			action: None,
			size: 20,
			color: Color::Fore(100),
		}
	}

	pub fn title(mut self, title: impl Into<String>) -> IconButton {
		self.title = Some(title.into());

		self
	}

	pub fn color(mut self, color: Color) -> IconButton {
		self.color = color;

		self
	}

	pub fn size(mut self, size: usize) -> IconButton {
		self.size = size;

		self
	}

	pub fn action(mut self, action: impl HasActionKey) -> IconButton {
		self.action = Some(action.get_action_key());

		self
	}
}
