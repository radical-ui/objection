use serde::{Deserialize, Serialize};

use crate::{ActionBatch, Color, ComponentId};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Label {
	_update_id: Option<ComponentId>,
	text: String,
	color: Option<Color>,
}

impl Label {
	pub fn new(text: impl Into<String>) -> Label {
		let mut label = Label::default();
		label.text = text.into();

		label
	}

	pub fn id(mut self, component_id: &ComponentId) -> Label {
		self._update_id = Some(component_id.to_owned());

		self
	}

	pub fn color(mut self, color: Color) -> Label {
		self.color = Some(color);

		self
	}

	pub fn set_text(batch: &mut ActionBatch, component_id: &ComponentId, text: impl Into<String>) {
		batch.update_prop(component_id, "text", text.into())
	}

	pub fn set_color(batch: &mut ActionBatch, component_id: &ComponentId, color: Color) {
		batch.update_prop(component_id, "color", color)
	}
}
