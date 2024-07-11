use std::env;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use crate::Component;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Window {
	pub title: String,
	pub theme: Option<Theme>,
	pub root_component: Component,
}

impl Window {
	pub fn as_json(&self) -> String {
		to_string(&self).unwrap()
	}

	pub fn as_html(&self, session_id: &Uuid) -> String {
		let base = match env::var("SVELTE_TOOLBOX_URL") {
			Ok(url) => url,
			Err(_) => {
				panic!("you must specify a SVELTE_TOOLBOX_URL");
			}
		};
		let title = &self.title;
		let root_ui = self.as_json();

		format!(
			"
			<!DOCTYPE html>
			<html lang=\"en\">
				<head>
					<meta charset=\"UTF-8\" />
					<link rel=\"icon\" type=\"image/svg+xml\" href=\"{base}/icon.svg\" />
					<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />

					<title>{title}</title>

					<script type=\"application/json\" id=\"root-ui\">{root_ui}</script>
					<script defer src=\"{base}/build.js\"></script>
				</head>
				<body data-session-id=\"{session_id}\">
					<div id=\"root\" style=\"display: none\"></div>
				</body>
			</html>
			"
		)
	}

	pub fn updated(title: impl Into<String>, component: impl Into<Component>) -> Window {
		Window {
			title: title.into(),
			theme: None,
			root_component: component.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ColorDefinition {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl ColorDefinition {
	pub fn new(red: u8, green: u8, blue: u8) -> ColorDefinition {
		ColorDefinition { red, green, blue }
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub enum SelectionMode {
	OptIn,
	#[default]
	OptOut,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Theme {
	pub round_base: bool,
	pub window_scrolling: bool,
	pub selection_mode: SelectionMode,
	pub light_palette: ColorPalette,
	pub dark_palette: ColorPalette,
	pub default_font: Option<String>,
	pub fancy_font: Option<String>,
}

impl Default for Theme {
	fn default() -> Theme {
		Theme {
			round_base: false,
			selection_mode: SelectionMode::default(),
			window_scrolling: false,
			default_font: None,
			fancy_font: None,
			light_palette: ColorPalette::default_light(),
			dark_palette: ColorPalette::default_dark(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ColorPalette {
	pub base: ColorDefinition,
	pub fore: ColorDefinition,
	pub decoration_fore: ColorDefinition,
	pub primary: ColorDefinition,
	pub secondary: ColorDefinition,
	pub danger: ColorDefinition,
	pub warn: ColorDefinition,
	pub success: ColorDefinition,
	pub notice: ColorDefinition,
}

impl ColorPalette {
	pub fn default_light() -> ColorPalette {
		ColorPalette {
			primary: ColorDefinition::new(28, 78, 216),
			secondary: ColorDefinition::new(251, 72, 94),
			base: ColorDefinition::new(255, 255, 255),
			fore: ColorDefinition::new(33, 37, 43),
			decoration_fore: ColorDefinition::new(255, 255, 255),
			success: ColorDefinition::new(4, 120, 87),
			danger: ColorDefinition::new(185, 28, 27),
			warn: ColorDefinition::new(194, 65, 11),
			notice: ColorDefinition::new(28, 78, 216),
		}
	}

	pub fn default_dark() -> ColorPalette {
		ColorPalette {
			primary: ColorDefinition::new(56, 189, 249),
			secondary: ColorDefinition::new(251, 72, 94),
			base: ColorDefinition::new(33, 37, 43),
			fore: ColorDefinition::new(255, 255, 255),
			decoration_fore: ColorDefinition::new(33, 37, 43),
			success: ColorDefinition::new(68, 208, 138),
			danger: ColorDefinition::new(255, 99, 72),
			warn: ColorDefinition::new(194, 65, 11),
			notice: ColorDefinition::new(56, 189, 249),
		}
	}
}

impl Default for ColorPalette {
	fn default() -> ColorPalette {
		ColorPalette::default_light()
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "opacity")]
pub enum Color {
	Primary(usize),
	Fore(usize),
	DecorationFore(usize),
	Base(usize),
	Danger(usize),
	Warn(usize),
	Success(usize),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum ColorType {
	Primary,
	Fore,
	DecorationFore,
	Base,
	Danger,
	Warn,
	Success,
}
