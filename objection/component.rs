use serde::{Deserialize, Serialize};

use crate::{Container, Label, Space};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "def")]
pub enum Component {
	Label(Label),
	Container(Container),
	Space(Space),
}

impl From<Label> for Component {
	fn from(value: Label) -> Self {
		Component::Label(value)
	}
}

impl From<Container> for Component {
	fn from(value: Container) -> Self {
		Component::Container(value)
	}
}

impl From<Space> for Component {
	fn from(value: Space) -> Self {
		Component::Space(value)
	}
}
