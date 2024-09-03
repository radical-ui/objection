use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Object {
	title: Option<String>,
	subtitle: Option<String>,
	description: Option<String>,
	icon: Option<String>,
	content: Vec<Content>,
	actions: Vec<Action>,
}

impl Object {
	pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
		self.title = Some(title.into());

		self
	}

	pub fn set_subtitle(&mut self, subtitle: impl Into<String>) -> &mut Self {
		self.subtitle = Some(subtitle.into());

		self
	}

	pub fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
		self.description = Some(description.into());

		self
	}

	pub fn set_icon(&mut self, icon: impl Into<String>) -> &mut Self {
		self.icon = Some(icon.into());

		self
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "def", rename_all = "snake_case")]
pub enum Content {
	Paragraph(Paragraph),
	Quote(Quote),
	ObjectPreview(ObjectPreview),
	CallToAction(CallToAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
	pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
	pub text: String,
	pub author: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectPreview {
	pub object_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallToAction {
	pub title: String,
	pub icon: Option<String>,
	pub target_object: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
	Danger,
	Success,
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
	pub id: String,
	pub kind: ActionKind,
	pub title: String,
	pub icon: Option<String>,
}
