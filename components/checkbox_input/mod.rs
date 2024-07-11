use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, HasActionKey};

/// A checkbox input, which can be either on or off.
///
/// At some point, this component should be combined with a sort of shared context on the frontend to connect with other checkboxes, define roots,
/// and be in an intermediate state.
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Action {
/// 	Foo
/// }
///
/// Flex::new(FlexKind::Column)
/// 	.auto_item(CheckboxInput::new("Allow tracking").initial_value(true).action(Action::Foo))
/// 	.auto_item(CheckboxInput::new("Allow tracking (disabled)").initial_value(false))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckboxInput {
	pub label: String,
	pub action: Option<ActionKey>,
	pub initial_value: bool,
}

impl CheckboxInput {
	pub fn new(label: impl Into<String>) -> CheckboxInput {
		CheckboxInput {
			label: label.into(),
			action: None,
			initial_value: false,
		}
	}

	pub fn action(mut self, action: impl HasActionKey) -> CheckboxInput {
		self.action = Some(action.get_action_key());

		self
	}

	pub fn initial_value(mut self, value: bool) -> CheckboxInput {
		self.initial_value = value;

		self
	}
}
