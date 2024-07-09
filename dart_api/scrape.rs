use reqwest::Url;

use crate::{Class, Constructor, Enum, Graph, Item};

pub async fn scrape(graph: &mut Graph) {
	loop {
		let qualified_name = match graph.next_unvisited_qualified_name_mut() {
			Some(item) => item,
			None => break,
		};

		match item {
			Item::Class(Class { constructors, .. }) => {}
			Item::Enum(enum_) => {}
		}
	}
}

async fn scrape_constructor(constructor: &mut Constructor, additional_urls: &mut Vec<Url>) {}

async fn scrape_enum(enum_: &mut Enum) {}
