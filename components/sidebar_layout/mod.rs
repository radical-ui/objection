use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{ActionKey, Component, HasActionKey, IconName, Image};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SidebarItem {
	pub title: String,
	pub icon: Option<IconName>,
	pub action: Option<ActionKey>,
}

impl SidebarItem {
	pub fn new(title: impl Into<String>) -> SidebarItem {
		SidebarItem {
			title: title.into(),
			icon: None,
			action: None,
		}
	}

	pub fn icon(mut self, icon: impl Into<IconName>) -> SidebarItem {
		self.icon = Some(icon.into());

		self
	}

	pub fn action(mut self, action: impl HasActionKey) -> SidebarItem {
		self.action = Some(action.get_action_key());

		self
	}
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SidebarGroup {
	pub name: String,
	pub items: Vec<SidebarItem>,
}

impl SidebarGroup {
	pub fn new(name: impl Into<String>) -> SidebarGroup {
		SidebarGroup {
			name: name.into(),
			items: Vec::new(),
		}
	}

	pub fn item(mut self, item: SidebarItem) -> SidebarGroup {
		self.items.push(item);

		self
	}
}

/// A sidebar application layout.
///
/// **Example**
///
/// ```rust
/// SidebarLayout::new("Abc Corp")
/// ```
///
/// ```rust
/// #[derive(HasActionKey, Serialize, Deserialize)]
/// enum Action {
/// 	Foo,
/// 	Bar,
/// }
///
/// SidebarLayout::new("Abc Corp")
/// 	.title_action(Action::Foo)
/// 	.logo(Image::new("https://github.githubassets.com/assets/3m-0151c2fda0ce.svg").width(30).height(30))
/// 	.action_item(SidebarItem::new("Tasks").icon("mdi-ab-testing").action(Action::Foo))
/// 	.action_item(SidebarItem::new("Activities").icon("mdi-ab-testing").action(Action::Bar))
/// 	.group(
/// 		SidebarGroup::new("Main")
/// 			.item(SidebarItem::new("Tasks").icon("mdi-ab-testing").action(Action::Foo))
/// 			.item(SidebarItem::new("Activities").icon("mdi-ab-testing").action(Action::Bar))
/// 	)
/// 	.group(
/// 		SidebarGroup::new("Records")
/// 			.item(SidebarItem::new("Tasks").icon("mdi-ab-testing").action(Action::Foo))
/// 			.item(SidebarItem::new("Activities").icon("mdi-ab-testing").action(Action::Bar))
/// 	)
/// 	.initial_action(Action::Foo)
/// 	.footer(Center::new().body("Za feetsies"))
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SidebarLayout {
	pub title: String,
	pub title_action: Option<ActionKey>,
	pub logo: Option<Image>,
	pub action_items: Vec<SidebarItem>,
	pub groups: Vec<SidebarGroup>,
	pub footer: Option<Box<Component>>,
	pub body: Option<Box<Component>>,
	pub initial_action: Option<ActionKey>,
}

impl SidebarLayout {
	pub fn new(title: impl Into<String>) -> SidebarLayout {
		SidebarLayout {
			title: title.into(),
			title_action: None,
			logo: None,
			action_items: Vec::new(),
			groups: Vec::new(),
			footer: None,
			body: None,
			initial_action: None,
		}
	}

	pub fn title_action(mut self, action: impl HasActionKey) -> SidebarLayout {
		self.title_action = Some(action.get_action_key());

		self
	}

	pub fn logo(mut self, logo: Image) -> SidebarLayout {
		self.logo = Some(logo);

		self
	}

	pub fn action_item(mut self, item: SidebarItem) -> SidebarLayout {
		self.action_items.push(item);

		self
	}

	pub fn group(mut self, group: SidebarGroup) -> SidebarLayout {
		self.groups.push(group);

		self
	}

	pub fn footer(mut self, component: impl Into<Component>) -> SidebarLayout {
		self.footer = Some(Box::new(component.into()));

		self
	}

	pub fn body(mut self, component: impl Into<Component>) -> SidebarLayout {
		self.body = Some(Box::new(component.into()));

		self
	}

	pub fn initial_action(mut self, action: impl HasActionKey) -> SidebarLayout {
		self.initial_action = Some(action.get_action_key());

		self
	}
}
