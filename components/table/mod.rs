use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TableColumn {
	name: String,
	expand: bool,
}

/// TODO
///
/// **Example**
///
/// ```rust
/// Table::new()
/// 	.column("Id")
/// 	.expanded_column("Name")
/// 	.column("")
/// 	.rows(Vec::from([
/// 		Vec::<Component>::from([ Label::new("82").into(), Label::new("Jason").into(), Button::new("View").size(ButtonSize::Small).into() ]),
/// 		Vec::<Component>::from([ Label::new("84").into(), Label::new("James").into(), Button::new("View").size(ButtonSize::Small).into() ]),
/// 		Vec::<Component>::from([ Label::new("103").into(), Label::new("Jeehoshofat Bartholemew, Duke of Northumberland, King of \"The Rose Garden\", the sixteenth").into(), Button::new("View").size(ButtonSize::Small).into() ]),
/// 	]))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Table {
	pub columns: Vec<TableColumn>,
	pub rows: Vec<Vec<Component>>,
}

impl Table {
	pub fn new() -> Table {
		Table {
			columns: Vec::new(),
			rows: Vec::new(),
		}
	}

	pub fn column(mut self, name: impl Into<String>) -> Table {
		self.columns.push(TableColumn {
			expand: false,
			name: name.into(),
		});

		self
	}

	pub fn expanded_column(mut self, name: impl Into<String>) -> Table {
		self.columns.push(TableColumn {
			expand: true,
			name: name.into(),
		});

		self
	}

	pub fn rows(mut self, rows: Vec<Vec<Component>>) -> Table {
		self.rows = rows;

		self
	}
}
