use markup5ever_rcdom::{Node as RawNode, NodeData};
use reqwest::header::CONTENT_DISPOSITION;
use std::{borrow::Borrow, fmt::Write, rc::Rc};

use crate::hierarchy::NodeHierarchyComponent;

#[derive(Debug)]
pub struct Node {
	pub(crate) bare: Rc<markup5ever_rcdom::Node>,
	pub(crate) hierarchy: Vec<NodeHierarchyComponent>,
}

impl Node {
	pub fn get_hierarchy_represenation(&self) -> String {
		let mut string = String::new();

		for component in &self.hierarchy {
			match component {
				NodeHierarchyComponent::Root => write!(string, ":root").unwrap(),
				NodeHierarchyComponent::Tag(tag) => write!(string, "{tag}").unwrap(),
				NodeHierarchyComponent::Class(class) => write!(string, ".{class}").unwrap(),
				NodeHierarchyComponent::Child => write!(string, " > ").unwrap(),
			}
		}

		string
	}

	pub fn get_text(&self) -> String {
		let mut string = String::new();

		push_text_content(&self.bare, &mut string);

		string
	}
}

fn push_text_content(raw_node: &RawNode, text: &mut String) {
	for child in raw_node.children.borrow().iter() {
		match &child.data {
			NodeData::Text { contents } => {
				text.push_str(&contents.borrow().to_string());
			}
			_ => (),
		}

		push_text_content(child, text)
	}
}
