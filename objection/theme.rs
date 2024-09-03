use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Theme {
	tab_bar: Option<TabBar>,
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
