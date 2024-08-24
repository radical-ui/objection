mod component;
mod container;
mod label;
mod space;

use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value, Value};

pub use component::*;
pub use container::*;
pub use label::*;
pub use space::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentId(String);

impl ComponentId {
	pub fn new() -> ComponentId {
		ComponentId(random::<u32>().to_string())
	}

	fn to_string(&self) -> String {
		self.0.clone()
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color(u8, u8, u8, u8);

impl Color {
	pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
		Color(red, green, blue, 255)
	}

	pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
		Color(red, green, blue, alpha)
	}
}

#[derive(Clone)]
pub struct ActionBatch {
	actions: Vec<Value>,
}

impl ActionBatch {
	fn new() -> ActionBatch {
		ActionBatch { actions: Vec::new() }
	}

	pub(crate) fn update_prop(&mut self, component_id: &ComponentId, name: &str, value: impl Serialize) {
		self.actions.push(json!({
			"operation": "Update",
			"id": component_id.to_string(),
			"state": {
				name: to_value(value).unwrap()
			}
		}))
	}
}
