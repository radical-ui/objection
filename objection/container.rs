use serde::{Deserialize, Serialize};

use crate::{ActionBatch, Color, Component, ComponentId};

#[derive(Debug, Serialize, Deserialize)]
pub enum Alignment {
	Start,
	Center,
	End,
}

pub struct Padding {
	top: f64,
	bottom: f64,
	right: f64,
	left: f64,
}

impl Padding {
	pub fn all(inset: f64) -> Padding {
		Padding {
			top: inset,
			bottom: inset,
			right: inset,
			left: inset,
		}
	}

	pub fn vertical(inset: f64) -> Padding {
		Padding {
			top: inset,
			bottom: inset,
			right: 0.0,
			left: 0.0,
		}
	}

	pub fn horizontal(inset: f64) -> Padding {
		Padding {
			top: 0.0,
			bottom: 0.0,
			right: inset,
			left: inset,
		}
	}

	pub fn top(mut self, inset: f64) -> Padding {
		self.top = inset;

		self
	}

	pub fn bottom(mut self, inset: f64) -> Padding {
		self.bottom = inset;

		self
	}

	pub fn right(mut self, inset: f64) -> Padding {
		self.right = inset;

		self
	}

	pub fn left(mut self, inset: f64) -> Padding {
		self.bottom = inset;

		self
	}
}

pub struct Shadow {
	radius: f64,
	color: Color,
	x: f64,
	y: f64,
}

impl Shadow {
	pub fn new(color: Color, radius: f64) -> Shadow {
		Shadow { radius, color, x: 0.0, y: 0.0 }
	}

	pub fn light() -> Shadow {
		Shadow {
			radius: 6.0,
			color: Color(0, 0, 0, 70),
			x: 0.0,
			y: 0.0,
		}
	}

	pub fn dark() -> Shadow {
		Shadow {
			radius: 10.0,
			color: Color(0, 0, 0, 150),
			x: 0.0,
			y: 0.0,
		}
	}

	pub fn x(mut self, x: f64) -> Shadow {
		self.x = x;

		self
	}

	pub fn y(mut self, y: f64) -> Shadow {
		self.y = y;

		self
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Container {
	_update_id: Option<ComponentId>,

	spacing: Option<f64>,
	children: Vec<Component>,
	is_horizontal: bool,
	alignment: Option<Alignment>,

	color: Option<Color>,

	padding_top: Option<f64>,
	padding_bottom: Option<f64>,
	padding_right: Option<f64>,
	padding_left: Option<f64>,

	corner_radius: Option<f64>,

	shadow_color: Option<Color>,
	shadow_x: Option<f64>,
	shadow_y: Option<f64>,
	shadow_radius: Option<f64>,

	border_color: Option<Color>,
	border_width: Option<f64>,
}

impl Container {
	pub fn new() -> Container {
		Container::default()
	}

	pub fn id(mut self, component_id: ComponentId) -> Container {
		self._update_id = Some(component_id);

		self
	}

	pub fn children(mut self, children: Vec<Component>) -> Container {
		self.children = children;

		self
	}

	pub fn child(mut self, child: impl Into<Component>) -> Container {
		self.children.push(child.into());

		self
	}

	pub fn spacing(mut self, spacing: f64) -> Container {
		self.spacing = Some(spacing);

		self
	}

	pub fn horizontal(mut self) -> Container {
		self.is_horizontal = true;

		self
	}

	pub fn horizontal_if(mut self, condition: bool) -> Container {
		self.is_horizontal = condition;

		self
	}

	pub fn alignment(mut self, alignment: Alignment) -> Container {
		self.alignment = Some(alignment);

		self
	}

	pub fn align_start(mut self) -> Container {
		self.alignment = Some(Alignment::Start);

		self
	}

	pub fn align_center(mut self) -> Container {
		self.alignment = Some(Alignment::Center);

		self
	}

	pub fn align_end(mut self) -> Container {
		self.alignment = Some(Alignment::End);

		self
	}

	pub fn color(mut self, color: Color) -> Container {
		self.color = Some(color);

		self
	}

	pub fn padding(mut self, padding: Padding) -> Container {
		self.padding_top = Some(padding.top);
		self.padding_bottom = Some(padding.bottom);
		self.padding_right = Some(padding.right);
		self.padding_left = Some(padding.left);

		self
	}

	pub fn corner_radius(mut self, radius: f64) -> Container {
		self.corner_radius = Some(radius);

		self
	}

	pub fn shadow(mut self, shadow: Shadow) -> Container {
		self.shadow_color = Some(shadow.color);
		self.shadow_radius = Some(shadow.radius);
		self.shadow_x = Some(shadow.x);
		self.shadow_y = Some(shadow.y);

		self
	}

	pub fn border(mut self, color: Color, width: f64) -> Container {
		self.border_color = Some(color);
		self.border_width = Some(width);

		self
	}

	pub fn set_children(batch: &mut ActionBatch, component_id: &ComponentId, children: Vec<Component>) {
		batch.update_prop(component_id, "children", children)
	}

	pub fn set_spacing(batch: &mut ActionBatch, component_id: &ComponentId, spacing: f64) {
		batch.update_prop(component_id, "spacing", spacing)
	}

	pub fn set_alignment(batch: &mut ActionBatch, component_id: &ComponentId, alignment: Alignment) {
		batch.update_prop(component_id, "alignment", alignment)
	}

	pub fn set_color(batch: &mut ActionBatch, component_id: &ComponentId, color: Color) {
		batch.update_prop(component_id, "color", color)
	}

	pub fn set_shadow(batch: &mut ActionBatch, component_id: &ComponentId, shadow: Shadow) {
		batch.update_prop(component_id, "shadow_color", shadow.color);
		batch.update_prop(component_id, "shadow_radius", shadow.radius);
		batch.update_prop(component_id, "shadow_x", shadow.x);
		batch.update_prop(component_id, "shadow_y", shadow.y);
	}

	pub fn set_padding(batch: &mut ActionBatch, component_id: &ComponentId, padding: Padding) {
		batch.update_prop(component_id, "padding_top", padding.top);
		batch.update_prop(component_id, "padding_bottom", padding.bottom);
		batch.update_prop(component_id, "padding_right", padding.right);
		batch.update_prop(component_id, "padding_left", padding.left);
	}

	pub fn set_border(batch: &mut ActionBatch, component_id: &ComponentId, color: Color, width: f64) {
		batch.update_prop(component_id, "border_color", color);
		batch.update_prop(component_id, "border_width", width);
	}
}
