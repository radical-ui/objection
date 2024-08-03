use crate::bindings::{ColorDefinition, ColorPalette, SelectionMode, Theme};

pub fn get_theme() -> Theme {
	Theme {
		round_base: false,
		default_font: None,
		window_scrolling: false,
		fancy_font: None,
		selection_mode: SelectionMode::OptIn,
		dark_palette: ColorPalette {
			primary: ColorDefinition::new(56.0, 189.0, 249.0),
			secondary: ColorDefinition::new(251.0, 72.0, 94.0),
			base: ColorDefinition::new(40.0, 40.0, 40.0),
			fore: ColorDefinition::new(255.0, 255.0, 255.0),
			decoration_fore: ColorDefinition::new(33.0, 37.0, 43.0),
			success: ColorDefinition::new(68.0, 208.0, 138.0),
			danger: ColorDefinition::new(255.0, 99.0, 72.0),
			warn: ColorDefinition::new(194.0, 65.0, 11.0),
			notice: ColorDefinition::new(56.0, 189.0, 249.0),
		},
		light_palette: ColorPalette {
			primary: ColorDefinition::new(28.0, 78.0, 216.0),
			secondary: ColorDefinition::new(251.0, 72.0, 94.0),
			base: ColorDefinition::new(255.0, 255.0, 255.0),
			fore: ColorDefinition::new(33.0, 37.0, 43.0),
			decoration_fore: ColorDefinition::new(255.0, 255.0, 255.0),
			success: ColorDefinition::new(4.0, 120.0, 87.0),
			danger: ColorDefinition::new(185.0, 28.0, 27.0),
			warn: ColorDefinition::new(194.0, 65.0, 11.0),
			notice: ColorDefinition::new(28.0, 78.0, 216.0),
		},
	}
}
