use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Value};
use std::{
	error::Error,
	fmt::{self, Display},
};
use uuid::Uuid;

use crate::Window;

#[derive(Debug)]
pub enum ActionKeyInferenceError {
	FailedToDecodeHex(String),
	FailedToDeserialize(Vec<u8>),
	NoAction,
}

impl Display for ActionKeyInferenceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ActionKeyInferenceError::FailedToDecodeHex(text) => write!(f, "Failed to decode hex: {text}"),
			ActionKeyInferenceError::FailedToDeserialize(bytes) => write!(f, "Couldn't deserialize bytes: {bytes:#?}"),
			ActionKeyInferenceError::NoAction => write!(f, "Expected an action, but none was supplied. Likely, a parent scope was messed up somehow."),
		}
	}
}

impl Error for ActionKeyInferenceError {}

#[derive(Debug)]
pub enum ActionPayloadError {
	NotEnough,
	Empty,
}

impl Display for ActionPayloadError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ActionPayloadError::NotEnough => write!(f, "Not enough action payloads were sent"),
			ActionPayloadError::Empty => write!(f, "An empty action payload was sent up"),
		}
	}
}

impl Error for ActionPayloadError {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Action {
	key: ActionKey,
	payload: Option<Value>,
}

impl Action {
	pub fn payload(mut self) -> Result<Value, ActionPayloadError> {
		self.payload.take().ok_or(ActionPayloadError::Empty)
	}
}

pub trait HasActionKey: Sized + Serialize + for<'de> Deserialize<'de> {
	fn get_action_key(&self) -> ActionKey {
		ActionKey(hex::encode(bincode::serialize(&self).unwrap()))
	}

	fn parse_from_action_key(key: &ActionKey) -> Result<Self, ActionKeyInferenceError> {
		let bytes = hex::decode(&key.0).map_err(|_| ActionKeyInferenceError::FailedToDecodeHex(key.0.to_string()))?;

		Ok(bincode::deserialize(&bytes).map_err(|_| ActionKeyInferenceError::FailedToDeserialize(bytes))?)
	}

	fn parse(action: &Action) -> Result<Self, ActionKeyInferenceError> {
		Self::parse_from_action_key(&action.key)
	}
}

impl HasActionKey for ActionKey {
	fn get_action_key(&self) -> ActionKey {
		self.clone()
	}

	fn parse(action: &Action) -> Result<Self, ActionKeyInferenceError> {
		Ok(action.key.clone())
	}

	fn parse_from_action_key(key: &ActionKey) -> Result<Self, ActionKeyInferenceError> {
		Ok(key.clone())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ActionKey(pub String);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum NoticeStyle {
	Error,
	Success,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Notice {
	message: String,
	style: NoticeStyle,
}

impl Notice {
	pub fn new(message: impl Display, style: NoticeStyle) -> Notice {
		Notice {
			message: message.to_string(),
			style,
		}
	}

	pub fn success(message: impl Display) -> Notice {
		Notice {
			message: message.to_string(),
			style: NoticeStyle::Success,
		}
	}

	pub fn error(message: impl Display) -> Notice {
		Notice {
			message: message.to_string(),
			style: NoticeStyle::Error,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "strategy", content = "data")]
pub enum UpdateAction {
	FullUpdate(Window),
	ComponentUpdate(u32, Value),
	AddNotice(Notice),
}

#[derive(Debug)]
pub struct Ui {
	updates: Vec<UpdateAction>,
	action_tree: Vec<Action>,
	new_access_token: Option<Uuid>,
}

impl Ui {
	pub fn new(action_tree: Vec<Action>) -> Ui {
		Ui {
			updates: Vec::new(),
			action_tree,
			new_access_token: None,
		}
	}

	pub fn refresh(&mut self, window: Window) {
		self.updates.push(UpdateAction::FullUpdate(window));
	}

	pub fn push_component_update(&mut self, target: u32, update: Value) {
		self.updates.push(UpdateAction::ComponentUpdate(target, update))
	}

	pub fn display_notice(&mut self, notice: Notice) {
		self.updates.push(UpdateAction::AddNotice(notice))
	}

	pub fn update_access_token(&mut self, token: Uuid) {
		self.new_access_token = Some(token)
	}

	pub fn get_access_token_update(&self) -> Option<&Uuid> {
		self.new_access_token.as_ref()
	}

	pub fn as_json(&self) -> String {
		to_string(&self.updates).unwrap()
	}

	pub fn take_action(&mut self) -> Result<Action, ActionKeyInferenceError> {
		self.action_tree.pop().ok_or(ActionKeyInferenceError::NoAction)
	}
}
