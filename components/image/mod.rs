use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ImageFit {
	Contain,
	Cover,
	Fill,
	None,
	ScaleDown,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ImagePosition {
	Bottom,
	Center,
	Left,
	LeftBottom,
	LeftTop,
	Right,
	RightBottom,
	RightTop,
	Top,
}

/// TODO
///
/// **Example**
///
/// ```rust
/// Image::new("https://images.unsplash.com/photo-1711436470690-cf49602d1cf1?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D")
/// 	.width(300)
/// 	.height(300)
/// 	.fit(ImageFit::Cover)
/// 	.decorate()
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Image {
	pub width: Option<usize>,
	pub height: Option<usize>,
	pub url: String,
	pub fit: ImageFit,
	pub position: ImagePosition,
	pub decorate: bool,
}

impl Image {
	pub fn new(url: impl Into<String>) -> Image {
		Image {
			width: None,
			height: None,
			url: url.into(),
			fit: ImageFit::Contain,
			position: ImagePosition::Center,
			decorate: false,
		}
	}

	pub fn width(mut self, width: usize) -> Image {
		self.width = Some(width);

		self
	}

	pub fn height(mut self, height: usize) -> Image {
		self.height = Some(height);

		self
	}

	pub fn fit(mut self, fit: ImageFit) -> Image {
		self.fit = fit;

		self
	}

	pub fn position(mut self, position: ImagePosition) -> Image {
		self.position = position;

		self
	}

	pub fn decorate(mut self) -> Image {
		self.decorate = true;

		self
	}
}
