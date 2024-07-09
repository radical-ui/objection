use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum DividerDirection {
	#[default]
	Horizontal,
	Vertical,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum DividerDistinction {
	Profound,
	#[default]
	Medium,
	Slight,
}

/// A visual divider, which can be horizontal or vertical, and can have varying distinction.
///
/// **Example**
///
/// ```rust
/// Flex::new(FlexKind::Column)
/// 	.gap(10)
/// 	.auto_item("Slight")
/// 	.auto_item(Divider::new().distinction(DividerDistinction::Slight))
/// 	.auto_item("Medium")
/// 	.auto_item(Divider::new().distinction(DividerDistinction::Medium))
/// 	.auto_item("Profound")
/// 	.auto_item(Divider::new().distinction(DividerDistinction::Profound))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Divider {
	direction: DividerDirection,
	distinction: DividerDistinction,
}

impl Divider {
	pub fn new() -> Divider {
		Divider {
			direction: DividerDirection::default(),
			distinction: DividerDistinction::default(),
		}
	}

	pub fn direction(mut self, direction: DividerDirection) -> Divider {
		self.direction = direction;

		self
	}

	pub fn distinction(mut self, distinction: DividerDistinction) -> Divider {
		self.distinction = distinction;

		self
	}
}
