use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
	pub title: Option<String>,
	pub subtitle: Option<String>,
	pub description: Option<String>,
	pub content: Vec<Content>,
	pub actions: Vec<Action>,
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
