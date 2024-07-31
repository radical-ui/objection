use rand::random;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_value, json, to_value, Value};
use std::{fmt::Display, future::Future, marker::PhantomData};
use thiserror::Error;

enum EventScope<'a> {
	Owned(String),
	Borrowed(&'a str),
}

pub struct Ui<'a> {
	event_path: &'a [String],
	event_path_pointer: usize,
	event_data: &'a mut Option<Value>,
	actions: &'a mut Vec<Value>,
	current_event_scope: Vec<EventScope<'a>>,
}

impl<'a> Ui<'a> {
	pub fn scope(&'a mut self, symbol: impl EventSymbol) -> Ui<'a> {
		let mut current_event_scope = self
			.current_event_scope
			.iter()
			.map(|item| match item {
				EventScope::Owned(string) => EventScope::Borrowed(string.as_str()),
				EventScope::Borrowed(str) => EventScope::Borrowed(str),
			})
			.collect::<Vec<_>>();

		current_event_scope.push(EventScope::Owned(symbol.to_string()));

		Ui {
			event_path: self.event_path,
			event_path_pointer: self.event_path_pointer + 1,
			event_data: self.event_data,
			actions: self.actions,
			current_event_scope,
		}
	}

	fn take_current_event_symbol(&mut self) -> Option<&str> {
		let data = self.event_path.get(self.event_path_pointer)?;
		self.event_path_pointer += 1;

		Some(data)
	}

	fn take_current_event_data(&mut self) -> Option<Value> {
		self.event_data.take()
	}

	fn push_action<T: Serialize>(&mut self, action: Action<T>) {
		self.actions.push(to_value(&action).unwrap());
	}
}

#[derive(Debug, Error)]
pub enum TakeMountEventError {
	#[error("found an empty path when trying to check for a mount event, which is never valid")]
	EmptyEventPath,
	#[error("event key stated that this is a mount event, but no event data was given, which is not valid")]
	NoEventData,
	#[error("event key stated that this is a mount event, but the event data didn't deserialize into the expected mount data structure; {serde_error}")]
	FailedToDeserializeMountData { serde_error: String },
}

#[derive(Deserialize)]
pub struct MountEventData {
	pub token: String,
}

pub struct RootUi {
	event_path: Vec<String>,
	event_data: Option<Value>,
	actions: Vec<Value>,
}

impl RootUi {
	fn from_event(event: RawEvent) -> RootUi {
		RootUi {
			event_path: event.key.event_path,
			event_data: Some(event.data),
			actions: Vec::new(),
		}
	}

	pub fn scope_main(&mut self) -> Ui {
		Ui {
			event_path: &self.event_path,
			event_path_pointer: 1,
			event_data: &mut self.event_data,
			actions: &mut self.actions,
			current_event_scope: Vec::from([EventScope::Owned("main".into())]),
		}
	}

	pub fn take_mount_event(&mut self) -> Result<Option<MountEventData>, TakeMountEventError> {
		let first_event = self.event_path.first().ok_or(TakeMountEventError::EmptyEventPath)?;

		Ok(if first_event == "root_mount" {
			Some(from_value(self.event_data.take().ok_or(TakeMountEventError::NoEventData)?).map_err(|inner| {
				TakeMountEventError::FailedToDeserializeMountData {
					serde_error: inner.to_string(),
				}
			})?)
		} else {
			None
		})
	}
}

#[derive(Debug, Error)]
#[error("Invalid request body. {serde_error}")]
struct RequestError {
	serde_error: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRequest {
	session_id: String,
	events: Vec<RawEvent>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEvent {
	key: RawEventKey,
	data: Value,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawEventKey {
	event_path: Vec<String>,
}

fn parse_request(json: Value) -> Result<RawRequest, RequestError> {
	from_value::<RawRequest>(json).map_err(|e| RequestError { serde_error: e.to_string() })
}

pub async fn handle_request<'a, Func, Output, Error>(request_body: Value, mut f: Func) -> Value
where
	Error: Display + Sized,
	Output: Future<Output = std::result::Result<RootUi, Error>>,
	Func: FnMut(String, RootUi) -> Output,
{
	let RawRequest { session_id, events } = match parse_request(request_body) {
		Ok(infos) => infos,
		Err(err) => {
			return json!([
				{
					"key": { "action_path": ["root_error"] },
					"data": err.to_string()
				}
			])
		}
	};

	let mut all_actions = Vec::new();

	for event in events {
		// really hate that I have to do this clone here, but it needs to be done until rust has better support for async closures
		// the concept is to ensure that session_id is borowed
		let actions = match f(session_id.clone(), RootUi::from_event(event)).await {
			Ok(ui) => ui.actions,
			Err(error) => Vec::from([json!({
				"key": {
					"action_path": ["root_error"],
					"data": error.to_string()
				}
			})]),
		};

		all_actions.extend(actions)
	}

	json!(all_actions)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventKey<T> {
	event_path: Vec<String>,
	debug_symbol: Option<String>,

	#[serde(skip)]
	_marker: PhantomData<T>,
}

#[derive(Debug, Error)]
pub enum TakeDataError {
	#[error(
		"this event path is different from the incomming event path;
		application should always validate the event path before taking the event data;
		this event path: {existing:?}; incomming event path: {incomming:?}"
	)]
	DifferingEventPaths { existing: Vec<String>, incomming: Vec<String> },

	#[error("tried to take event data, but it was already taken; this is probably caused by calling the EventKey::take_data function more than once in a single event loop cycle")]
	DataAlreadyTaken,

	#[error("failed to deserialize event data according the the pre-specified type; {serde_error}")]
	FailedToDeserialize { serde_error: String },
}

impl<T: DeserializeOwned> EventKey<T> {
	pub fn take_data(&self, ui: &mut Ui) -> Result<T, TakeDataError> {
		if self.event_path.len() != ui.event_path.len() {
			return Err(TakeDataError::DifferingEventPaths {
				existing: self.event_path.clone(),
				incomming: ui.event_path.to_vec(),
			});
		}

		for (index, symbol) in self.event_path.iter().enumerate() {
			let expected_symbol = ui.event_path.get(index).expect("this should never happen because of the length check above");

			if expected_symbol != symbol {
				return Err(TakeDataError::DifferingEventPaths {
					existing: self.event_path.clone(),
					incomming: ui.event_path.to_vec(),
				});
			}
		}

		let raw_data = ui.take_current_event_data().ok_or(TakeDataError::DataAlreadyTaken)?;

		let data = from_value(raw_data).map_err(|inner| TakeDataError::FailedToDeserialize {
			serde_error: inner.to_string(),
		})?;

		Ok(data)
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionKey<T> {
	action_path: Vec<String>,
	debug_symbol: Option<String>,

	#[serde(skip)]
	_marker: PhantomData<T>,
}

impl<T: Serialize + Clone> ActionKey<T> {
	pub fn create() -> ActionKey<T> {
		ActionKey {
			action_path: Vec::from([random::<u64>().to_string()]),
			debug_symbol: None,
			_marker: PhantomData,
		}
	}

	pub fn with_debug_symbol(mut self, data: impl Into<String>) -> Self {
		self.debug_symbol = Some(data.into());

		self
	}

	pub fn emit(&self, data: T, ui: &mut Ui) {
		ui.push_action(Action { key: self.to_owned(), data });
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event<T> {
	key: EventKey<T>,
	data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action<T> {
	key: ActionKey<T>,
	data: T,
}

#[derive(Debug, Error)]
pub enum FromStringError {
	#[error("failed to decode hex; {inner_error}; the following text is what we tried to parse: {hex}")]
	FailedToDecodeHex { hex: String, inner_error: String },

	#[error("failed to deserialize from raw bytes; {serde_error}; the following bytes are what we tried to deserialize: {bytes:?}")]
	FailedToDeserialize { bytes: Vec<u8>, serde_error: String },
}

#[derive(Debug, Error)]
pub enum ParseError {
	#[error("failed to parse the symbol from a string; {0}")]
	FromStringError(FromStringError),

	#[error(
		"tried to parse the next symbol, but there are none left; this could be a user error, but it could also be caused by not calling Ui::scope somewhere"
	)]
	NoSymbolsLeft,
}

pub trait EventSymbol: Sized + Serialize + for<'de> Deserialize<'de> {
	fn to_string(&self) -> String {
		hex::encode(bincode::serialize(&self).unwrap())
	}

	fn from_string(string: &str) -> Result<Self, FromStringError> {
		let bytes = hex::decode(string).map_err(|inner| FromStringError::FailedToDecodeHex {
			hex: string.to_string(),
			inner_error: inner.to_string(),
		})?;

		bincode::deserialize(&bytes).map_err(|inner| FromStringError::FailedToDeserialize {
			bytes: bytes.to_vec(),
			serde_error: inner.to_string(),
		})
	}

	fn parse(ui: &mut Ui) -> Result<Self, ParseError> {
		let symbol = ui.take_current_event_symbol().ok_or(ParseError::NoSymbolsLeft)?;

		Self::from_string(symbol).map_err(|inner| ParseError::FromStringError(inner))
	}
}
