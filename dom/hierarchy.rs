use markup5ever_rcdom::{Node as RawNode, NodeData};
use rutils::InnerPushVecExt;
use std::rc::Rc;

use crate::node::Node;

/// A non ambigious value containing owned components comparable to `ConcreteSelector`.
#[derive(Debug, Clone)]
pub enum NodeHierarchyComponent {
	Root,
	Tag(String),
	Class(String),
	Child,
}

/// A non-ambigious constraint that can be used for making selections. Generally, this is constructed from the more generic
/// and user-friendly `Selector` via `deambiguify_selectors`.
#[derive(Debug)]
pub enum ConcreteSelector<'a> {
	Root,
	Tag(&'a str),
	Class(&'a str),
	Child,
}

/// A constraint that can be used for making selections.
#[derive(Debug, Clone, Copy)]
pub enum Selector<'a> {
	Root,
	Tag(&'a str),
	Class(&'a str),
	DirectChild,
	AnyChild,
}

/// Convert an array of selectors into a vector of concrete selector groups. The sub-vector boundaries are used in place
// of the ambigious `Selector::AnyChild`. Selectors are also placed in a deterministic order.
pub fn deambiguify_selectors<'a>(selectors: &'a [Selector<'a>]) -> Vec<Vec<ConcreteSelector<'a>>> {
	let mut concrete_selectors = Vec::new();
	let mut current_classes = Vec::new();

	for selector in selectors {
		match selector {
			Selector::Root => concrete_selectors.inner_push(ConcreteSelector::Root),
			Selector::Tag(name) => concrete_selectors.inner_push(ConcreteSelector::Tag(name)),
			Selector::Class(name) => current_classes.push(name),
			Selector::DirectChild | Selector::AnyChild => {
				current_classes.sort();

				for class in current_classes.drain(..) {
					concrete_selectors.inner_push(ConcreteSelector::Class(class));
				}

				if let Selector::AnyChild = selector {
					concrete_selectors.push(Vec::new())
				}

				if let Selector::DirectChild = selector {
					concrete_selectors.inner_push(ConcreteSelector::Child);
				}
			}
		}
	}

	current_classes.sort();

	for class in current_classes.drain(..) {
		concrete_selectors.inner_push(ConcreteSelector::Class(class));
	}

	concrete_selectors
}

pub fn flatten_tree(node: Rc<RawNode>, parent_hierarchy: &[NodeHierarchyComponent], nodes: &mut Vec<Node>) {
	let mut hierarchy = parent_hierarchy.to_vec();

	match &node.data {
		NodeData::Document => hierarchy.push(NodeHierarchyComponent::Root),
		NodeData::Element { name, attrs, .. } => {
			hierarchy.push(NodeHierarchyComponent::Tag(name.local.to_string()));

			let mut classes = Vec::new();

			for attr in attrs.borrow().iter() {
				if &attr.name.local == "class" {
					classes.append(&mut attr.value.split_whitespace().map(|item| item.to_string()).collect())
				}
			}

			classes.sort();

			for class in classes {
				hierarchy.push(NodeHierarchyComponent::Class(class))
			}
		}
		_ => return,
	}

	hierarchy.push(NodeHierarchyComponent::Child);

	for child in node.children.borrow().iter() {
		flatten_tree(child.clone(), &hierarchy, nodes);
	}

	hierarchy.pop();
	nodes.push(Node { bare: node, hierarchy })
}
