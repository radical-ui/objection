use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CornerRounding {
	Sharp,
	#[default]
	Round,
	ExtraRound,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

impl Color {
	pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
		Color { red, green, blue, alpha: 255 }
	}

	pub fn with_alpha(mut self, alpha: u8) -> Color {
		self.alpha = alpha;

		self
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Surface {
	pub background_color_1: Color,
	pub background_color_2: Color,
	pub background_color_3: Color,
	pub background_color_4: Color,

	pub foreground_color_1: Color,
	pub foreground_color_2: Color,
	pub foreground_color_3: Color,
	pub foreground_color_4: Color,

	pub primary_color_1: Color,
	pub primary_color_2: Color,
	pub primary_color_3: Color,
	pub primary_color_4: Color,

	pub glow_color: Option<Color>,
}

impl Default for Surface {
	fn default() -> Self {
		Surface {
			background_color_1: Color::rgb(0, 0, 0).with_alpha(255),
			background_color_2: Color::rgb(20, 20, 20).with_alpha(255),
			background_color_3: Color::rgb(30, 30, 30).with_alpha(255),
			background_color_4: Color::rgb(40, 40, 40).with_alpha(255),

			foreground_color_1: Color::rgb(255, 255, 255).with_alpha(255),
			foreground_color_2: Color::rgb(255, 255, 255).with_alpha(255),
			foreground_color_3: Color::rgb(255, 255, 255).with_alpha(255),
			foreground_color_4: Color::rgb(255, 255, 255).with_alpha(255),

			primary_color_1: Color::rgb(63, 136, 226).with_alpha(210),
			primary_color_2: Color::rgb(63, 136, 226).with_alpha(170),
			primary_color_3: Color::rgb(63, 136, 226).with_alpha(140),
			primary_color_4: Color::rgb(63, 136, 226).with_alpha(80),

			glow_color: None,
		}
	}
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Theme {
	tab_bar: Option<TabBar>,
	corner_rounding: CornerRounding,
	light_surfaces: HashMap<String, Surface>,
	dark_surfaces: HashMap<String, Surface>,
	default_light_surface: Surface,
	default_dark_surface: Surface,
	navigation_surface: Option<String>,
}

impl Theme {
	pub(crate) fn get_entry_object_ids(&self) -> Vec<String> {
		let mut ids = Vec::new();

		if let Some(tab_bar) = &self.tab_bar {
			for id in &tab_bar.objects {
				ids.push(id.clone())
			}
		}

		ids
	}

	pub fn set_tab_bar(&mut self, tab_bar: TabBar) -> &mut Self {
		self.tab_bar = Some(tab_bar);

		self
	}

	pub fn set_corner_rounding(&mut self, corner_rounding: CornerRounding) -> &mut Self {
		self.corner_rounding = corner_rounding;

		self
	}

	pub fn set_surface(&mut self, name: impl Into<String>, light: Surface, dark: Surface) -> &mut Self {
		let name = name.into();

		self.light_surfaces.insert(name.clone(), light);
		self.dark_surfaces.insert(name, dark);

		self
	}

	pub fn set_uniform_surface(&mut self, name: impl Into<String>, surface: Surface) -> &mut Self {
		let name = name.into();

		self.light_surfaces.insert(name.clone(), surface.clone());
		self.dark_surfaces.insert(name, surface);

		self
	}

	pub fn set_default_surface(&mut self, light: Surface, dark: Surface) -> &mut Self {
		self.default_light_surface = light;
		self.default_dark_surface = dark;

		self
	}

	pub fn set_default_uniform_surface(&mut self, surface: Surface) -> &mut Self {
		self.default_light_surface = surface.clone();
		self.default_dark_surface = surface;

		self
	}
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TabBar {
	objects: Vec<String>,
}

impl TabBar {
	pub fn add_object(&mut self, object_id: impl Into<String>) -> &mut TabBar {
		self.objects.push(object_id.into());

		self
	}
}
