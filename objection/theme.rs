use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
	pub tab_bar: Option<TabBar>,
}

impl Theme {
	pub(crate) fn get_entry_object_ids(&self) -> Vec<String> {
		let mut ids = Vec::new();

		if let Some(tab_bar) = &self.tab_bar {
			for item in &tab_bar.items {
				ids.push(item.object_id.clone())
			}
		}

		ids
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabBar {
	pub items: Vec<TabBarItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabBarItem {
	pub icon: String,
	pub label: String,
	pub object_id: String,
}
