use rand::random;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_value;

use crate::{Component, Ui};

/// A boundary that allows it's children to be updated without beaming down a new window.
///
/// You can optionally set a child to be displayed until an update is sent. Once an update has been sent has been sent,
/// the original child will never be rendered.
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateBoundary {
	id: u32,
	child: Option<Box<Component>>,
}

impl UpdateBoundary {
	pub fn new(hook: &UpdateHook) -> UpdateBoundary {
		UpdateBoundary { id: hook.0, child: None }
	}

	pub fn child(mut self, child: impl Into<Component>) -> UpdateBoundary {
		self.child = Some(Box::new(child.into()));

		self
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHook(u32);

impl UpdateHook {
	pub fn new() -> UpdateHook {
		UpdateHook(random())
	}

	pub fn update(&self, plan: &mut Ui, child: impl Into<Component>) {
		plan.push_component_update(self.0, to_value(child.into()).unwrap())
	}
}

impl Default for UpdateHook {
	fn default() -> Self {
		UpdateHook::new()
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SurgicalUpdate {
	pub id: u32,
	pub component: Component,
}
