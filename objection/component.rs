use serde::{Deserialize, Serialize};

use crate::{Container, Divider, Label, Space};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "def")]
pub enum Component {
	Label(Label),
	Container(Container),
	Space(Space),
	Divider(Divider),
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

impl From<Divider> for Component {
	fn from(value: Divider) -> Self {
		Component::Divider(value)
	}
}
