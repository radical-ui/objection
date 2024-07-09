use rand::random;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::{ActionKey, HasActionKey, IconName, Ui};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TextInputHook(u32);

impl TextInputHook {
	pub fn new() -> TextInputHook {
		TextInputHook(random())
	}

	pub fn set_dropdown_options(&self, ui: &mut Ui, options: impl Into<Vec<DropdownOption>>) {
		ui.push_component_update(self.0, to_value(TextInputUpdate::SetDropdownOptions { options: options.into() }).unwrap())
	}

	pub fn set_validity(&self, ui: &mut Ui, validity: InputValidity, message: impl Into<String>) {
		ui.push_component_update(
			self.0,
			to_value(TextInputUpdate::SetValidity {
				validity,
				message: message.into(),
			})
			.unwrap(),
		)
	}
}

impl Default for TextInputHook {
	fn default() -> Self {
		TextInputHook::new()
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "content")]
pub enum TextInputUpdate {
	SetValidity { validity: InputValidity, message: String },
	SetDropdownOptions { options: Vec<DropdownOption> },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
pub enum InputValidity {
	Valid,
	Invalid,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DropdownOption {
	pub id: String,
	pub title: String,
	pub description: Option<String>,
	pub informative: Option<String>,
	pub is_disabled: bool,
}

impl DropdownOption {
	pub fn new(id: impl Into<String>, title: impl Into<String>) -> DropdownOption {
		DropdownOption {
			id: id.into(),
			title: title.into(),
			description: None,
			informative: None,
			is_disabled: false,
		}
	}

	pub fn description(mut self, description: impl Into<String>) -> DropdownOption {
		self.description = Some(description.into());

		self
	}

	pub fn informative(mut self, informative: impl Into<String>) -> DropdownOption {
		self.informative = Some(informative.into());

		self
	}

	pub fn is_disabled(mut self) -> DropdownOption {
		self.is_disabled = true;

		self
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum TextInputRole {
	#[default]
	Plain,
	Password,
	Email,
	Search,
	Url,
	Tel,
	Numeric,
	Decimal,
}

/// A text input. If no change action, blur action, or dropdown selection action is supplied, the input will be disabled.
///
/// If some initial dropdown options are supplied, but no `change_action` is supplied, the dropdown options will be sorted
/// locally. If a `change_action` is supplied, the server is expected to send down a new list of dropdown options.
///
/// If no `option_selection_action` is supplied, the selected dropdown options will simply replace the input value, triggering the
/// default value update behavior.
///
/// `allow_multiple_options` has no effect if an `option_selected_option` is not supplied. If it is, more that one option can be
/// selected.
///
/// **Example**
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Event {
/// 	InputChanged,
/// 	InputBlurred,
/// 	OptionSelected,
/// 	Submit
/// }
///
/// Padding::all(30).body(
/// 	Flex::new(FlexKind::Column)
/// 		.gap(20)
/// 		.auto_item(TextInput::new("Username").change_action(Event::InputChanged).submit_action(Event::Submit))
/// 		.auto_item(TextInput::new("Password").role(TextInputRole::Password).blur_action(Event::InputBlurred).submit_action(Event::Submit))
/// 		.auto_item(TextInput::new("With Initial Value").initial_value("Hello there!").blur_action(Event::InputBlurred).submit_action(Event::Submit))
/// 		.auto_item(TextInput::new("Email (disabled)").submit_action(Event::Submit).role(TextInputRole::Email).leading_icon("mdi-ab-testing"))
/// 		.auto_item(
/// 			TextInput::new("Dropdown with client filtering")
/// 				.role(TextInputRole::Email)
/// 				.blur_action(Event::InputBlurred)
/// 				.submit_action(Event::Submit)
/// 				.initial_dropdown_options(Vec::from([
/// 					DropdownOption::new(Uuid::new_v4(), "Option 1"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 2"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 3"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 4"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 5"),
/// 				]))
/// 		)
/// 		.auto_item(
/// 			TextInput::new("Dropdown with server filtering")
/// 				.role(TextInputRole::Email)
/// 				.change_action(Event::InputChanged)
/// 				.submit_action(Event::Submit)
/// 				.initial_dropdown_options(Vec::from([
/// 					DropdownOption::new(Uuid::new_v4(), "Option 1"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 2"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 3").is_disabled(),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 4"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 5"),
/// 				]))
/// 		)
///			.auto_item(
/// 			TextInput::new("Dropdown without free text and client filtering")
/// 				.role(TextInputRole::Email)
/// 				.option_selected_action(Event::OptionSelected)
/// 				.submit_action(Event::Submit)
/// 				.initial_dropdown_options(Vec::from([
/// 					DropdownOption::new(Uuid::new_v4(), "Option 1"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 2"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 3"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 4"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 5"),
/// 				]))
/// 		)
/// 		.auto_item(
/// 			TextInput::new("Dropdown without free text and client filtering and multiple")
/// 				.role(TextInputRole::Email)
/// 				.option_selected_action(Event::OptionSelected)
/// 				.submit_action(Event::Submit)
/// 				.multiple()
/// 				.initial_dropdown_options(Vec::from([
/// 					DropdownOption::new(Uuid::new_v4(), "Option 1"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 2"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 3"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 4"),
/// 					DropdownOption::new(Uuid::new_v4(), "Option 5"),
/// 				]))
/// 		)
/// )
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TextInput {
	pub change_action: Option<ActionKey>,
	pub blur_action: Option<ActionKey>,
	pub option_selected_action: Option<ActionKey>,
	pub submit_action: Option<ActionKey>,
	pub leading_icon: Option<IconName>,
	pub trailing_icon: Option<IconName>,
	pub label: String,
	pub role: TextInputRole,
	pub initial_value: Option<String>,
	pub initial_dropdown_options: Option<Vec<DropdownOption>>,
	pub initial_selected_options: Option<Vec<String>>,
	pub update_hook: Option<TextInputHook>,
	pub multiple: bool,
}

impl TextInput {
	pub fn new(label: impl Into<String>) -> TextInput {
		TextInput {
			change_action: None,
			blur_action: None,
			option_selected_action: None,
			submit_action: None,
			label: label.into(),
			leading_icon: None,
			trailing_icon: None,
			role: TextInputRole::default(),
			initial_value: None,
			initial_dropdown_options: None,
			initial_selected_options: None,
			update_hook: None,
			multiple: false,
		}
	}

	pub fn change_action(mut self, action: impl HasActionKey) -> TextInput {
		self.change_action = Some(action.get_action_key());

		self
	}

	pub fn blur_action(mut self, action: impl HasActionKey) -> TextInput {
		self.blur_action = Some(action.get_action_key());

		self
	}

	pub fn option_selected_action(mut self, action: impl HasActionKey) -> TextInput {
		self.option_selected_action = Some(action.get_action_key());

		self
	}

	pub fn submit_action(mut self, action: impl HasActionKey) -> TextInput {
		self.submit_action = Some(action.get_action_key());

		self
	}

	pub fn initial_dropdown_options(mut self, options: Vec<DropdownOption>) -> TextInput {
		self.initial_dropdown_options = Some(options);

		self
	}

	pub fn leading_icon(mut self, icon_name: impl Into<IconName>) -> TextInput {
		self.leading_icon = Some(icon_name.into());

		self
	}

	pub fn trailing_icon(mut self, icon_name: impl Into<IconName>) -> TextInput {
		self.trailing_icon = Some(icon_name.into());

		self
	}

	pub fn role(mut self, role: TextInputRole) -> TextInput {
		self.role = role;

		self
	}

	pub fn initial_value(mut self, value: impl Into<String>) -> TextInput {
		self.initial_value = Some(value.into());

		self
	}

	pub fn initial_selected_options(mut self, value: Vec<String>) -> TextInput {
		self.initial_selected_options = Some(value);

		self
	}

	pub fn update_hook(mut self, hook: TextInputHook) -> TextInput {
		self.update_hook = Some(hook);

		self
	}

	pub fn multiple(mut self) -> TextInput {
		self.multiple = true;

		self
	}
}
