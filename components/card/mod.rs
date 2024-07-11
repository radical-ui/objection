use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ColorType, Component};

/// A card that can optionally be colored.
///
/// **Examples**
///
/// ```rust
/// Padding::all(10).body(Card::new().body(Label::new("Hey! I am a card!")))
/// ```
///
/// ```rust
/// Padding::all(10).body(Card::new().body(Label::new("Hey! I am a red card!")).color(ColorType::Danger))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Card {
	body: Option<Box<Component>>,
	color: ColorType,
}

impl Card {
	pub fn new() -> Card {
		Card {
			body: None,
			color: ColorType::Fore,
		}
	}

	pub fn body(mut self, body: impl Into<Component>) -> Card {
		self.body = Some(Box::new(body.into()));

		self
	}

	pub fn color(mut self, color: ColorType) -> Card {
		self.color = color;

		self
	}
}
