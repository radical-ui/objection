mod hierarchy;
mod node;
mod query;

use hierarchy::{deambiguify_selectors, flatten_tree};
use html5ever::{parse_document, tendril::TendrilSink, ParseOpts};
use markup5ever_rcdom::RcDom;
use query::concrete_selector_groups_do_match;

pub use hierarchy::Selector;
pub use node::Node;

#[derive(Debug)]
pub struct Dom {
	nodes: Vec<Node>,
}

impl Dom {
	pub fn from_string(html: String) -> Dom {
		let parser = parse_document(RcDom::default(), ParseOpts::default());
		let dom = parser.one(html);
		let mut nodes = Vec::new();

		flatten_tree(dom.document, &[], &mut nodes);

		Dom { nodes }
	}

	pub fn query(&self, query: &[Selector<'_>]) -> Vec<&Node> {
		let mut matching_nodes = Vec::new();
		let concrete_selectors = deambiguify_selectors(query);

		for node in &self.nodes {
			if concrete_selector_groups_do_match(&node.hierarchy, &concrete_selectors) {
				matching_nodes.push(node)
			}
		}

		matching_nodes
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const TEST_HTML: &str = include_str!("./test.html");

	#[test]
	fn a_basic_query_works() {
		let page = Dom::from_string(TEST_HTML.to_string());

		let results = page.query(&[Selector::Tag("body")]);

		assert_eq!(results.len(), 1);
		assert_eq!(
			results.first().unwrap().get_hierarchy_represenation(),
			":root > html.colibri > body.no-js.sidebar-visible"
		);
	}

	#[test]
	fn a_query_with_direct_children_works() {
		let page = Dom::from_string(TEST_HTML.to_string());

		let results = page.query(&[
			Selector::Tag("ol"),
			Selector::DirectChild,
			Selector::Tag("li"),
			Selector::DirectChild,
			Selector::Tag("a"),
			Selector::DirectChild,
			Selector::Tag("strong"),
		]);

		assert_eq!(results.len(), 15);
		assert_eq!(
			results.first().unwrap().get_hierarchy_represenation(),
			":root > html.colibri > body.no-js.sidebar-visible > div > nav.sidebar > div.sidebar-scrollbox > ol.chapter > li.chapter-item.expanded > a > strong"
		);
	}

	#[test]
	fn an_ambigious_query_works() {
		let page = Dom::from_string(TEST_HTML.to_string());

		let results = page.query(&[Selector::Tag("html"), Selector::AnyChild, Selector::Tag("title")]);

		assert_eq!(results.len(), 1);
		assert_eq!(results.first().unwrap().get_hierarchy_represenation(), ":root > html.colibri > head > title");
	}

	#[test]
	fn a_double_ambigious_query() {
		let page = Dom::from_string(TEST_HTML.to_string());

		let results = page.query(&[
			Selector::Tag("body"),
			Selector::AnyChild,
			Selector::Tag("nav"),
			Selector::AnyChild,
			Selector::Tag("a"),
		]);

		assert_eq!(results.len(), 18);
		assert_eq!(results.first().unwrap().get_hierarchy_represenation(), ":root > html.colibri > body.no-js.sidebar-visible > div > nav.sidebar > div.sidebar-scrollbox > ol.chapter > li.affix.chapter-item.expanded > a.active");
	}

	#[test]
	fn class_only_query() {
		let page = Dom::from_string(TEST_HTML.to_string());

		let results = page.query(&[Selector::Class("fa"), Selector::Class("fa-edit")]);

		assert_eq!(results.len(), 1);
		assert_eq!(results.first().unwrap().get_hierarchy_represenation(), ":root > html.colibri > body.no-js.sidebar-visible > div > div.page-wrapper > div.page > div.menu-bar.sticky > div.right-buttons > a > i.fa.fa-edit");
	}
}
