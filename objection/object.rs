use serde::{Deserialize, Serialize};

use crate::Surface;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Object {
	title: Option<String>,
	subtitle: Option<String>,
	icon: Option<String>,
	image: Option<String>,
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

	pub fn set_icon(&mut self, icon: impl Into<String>) -> &mut Self {
		self.icon = Some(icon.into());

		self
	}

	pub fn set_image(&mut self, url: impl Into<String>) -> &mut Self {
		self.image = Some(url.into());

		self
	}

	pub fn push_content(&mut self, content: impl Into<Content>) -> &mut Self {
		self.content.push(content.into());

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
	ObjectGroup(ObjectGroup),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paragraph {
	pub text: String,
}

impl From<Paragraph> for Content {
	fn from(value: Paragraph) -> Self {
		Content::Paragraph(value)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
	pub text: String,
	pub attribution: String,
	pub surface: String,
	pub attribution_surface: String,
}

impl From<Quote> for Content {
	fn from(value: Quote) -> Self {
		Content::Quote(value)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectPreview {
	pub object_id: String,
	pub surface: String,
}

impl From<ObjectPreview> for Content {
	fn from(value: ObjectPreview) -> Self {
		Content::ObjectPreview(value)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallToAction {
	pub title: String,
	pub icon: Option<String>,
	pub target_object: String,
	pub surface: String,
}

impl From<CallToAction> for Content {
	fn from(value: CallToAction) -> Self {
		Content::CallToAction(value)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectGroup {
	pub title: String,
	pub description: Option<String>,
	pub objects: Vec<String>,
	pub surface: Option<Surface>,
}

impl From<ObjectGroup> for Content {
	fn from(value: ObjectGroup) -> Self {
		Content::ObjectGroup(value)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
	Primary,
	Danger,
	Success,
	Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
	pub kind: ActionKind,
	pub title: String,
	pub icon: Option<String>,
	pub process: ActionProcess,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "def", rename_all = "snake_case")]
pub enum ActionProcess {
	PerformOperation { key: String },
	ShowObject { id: String },
}
