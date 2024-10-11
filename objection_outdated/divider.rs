use serde::{Deserialize, Serialize};

use crate::{ActionBatch, Color, ComponentId};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Divider {
	_update_id: Option<ComponentId>,

	thickness: Option<f64>,
	color: Option<Color>,
}

impl Divider {
	pub fn new() -> Divider {
		Divider::default()
	}

	pub fn id(mut self, component_id: &ComponentId) -> Divider {
		self._update_id = Some(component_id.to_owned());

		self
	}

	pub fn color(mut self, color: Color) -> Divider {
		self.color = Some(color);

		self
	}

	pub fn thickness(mut self, thickness: f64) -> Divider {
		self.thickness = Some(thickness);

		self
	}

	pub fn set_color(batch: &mut ActionBatch, component_id: &ComponentId, color: Color) {
		batch.update_prop(&component_id, "color", color)
	}

	pub fn set_thickness(batch: &mut ActionBatch, component_id: &ComponentId, thickness: f64) {
		batch.update_prop(&component_id, "thickness", thickness)
	}
}
