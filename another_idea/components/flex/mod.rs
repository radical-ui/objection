use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FlexKind {
	Row,
	Column,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum FlexAlign {
	#[default]
	Stretch,
	Center,
	Start,
	End,
	Baseline,
	SafeCenter,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum FlexJustify {
	Center,
	SafeCenter,
	Start,
	End,
	SpaceBetween,
	SpaceAround,
	SpaceEvenly,
	#[default]
	Stretch,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FlexGrowth {
	Auto,
	Expand,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FlexItem {
	pub growth: FlexGrowth,
	pub component: Component,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Flex {
	pub kind: FlexKind,
	pub gap: usize,
	pub align: FlexAlign,
	pub justify: FlexJustify,
	pub items: Vec<FlexItem>,
}

/// A flex container.
///
/// ```rust
/// Flex::new(FlexKind::Row)
/// 	.gap(10)
/// 	.expand_item( Card::new().body(Label::new("First")))
/// 	.expand_item( Card::new().body(Label::new("Second")))
/// ```
impl Flex {
	pub fn new(kind: FlexKind) -> Flex {
		Flex {
			kind,
			gap: 0,
			align: FlexAlign::default(),
			justify: FlexJustify::default(),
			items: Vec::new(),
		}
	}

	pub fn justify(mut self, justify: FlexJustify) -> Flex {
		self.justify = justify;

		self
	}

	pub fn align(mut self, align: FlexAlign) -> Flex {
		self.align = align;

		self
	}

	pub fn items(mut self, items: impl Into<Vec<FlexItem>>) -> Flex {
		self.items = items.into();

		self
	}

	pub fn auto_item(mut self, component: impl Into<Component>) -> Flex {
		self.items.push(FlexItem {
			growth: FlexGrowth::Auto,
			component: component.into(),
		});

		self
	}

	pub fn expand_item(mut self, component: impl Into<Component>) -> Flex {
		self.items.push(FlexItem {
			growth: FlexGrowth::Expand,
			component: component.into(),
		});

		self
	}

	pub fn gap(mut self, gap: usize) -> Flex {
		self.gap = gap;

		self
	}
}
