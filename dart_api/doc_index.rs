use dom::{Dom, Selector};
use error_stack::ResultExt;
use log::warn;
use reqwest::Url;
use serde_json::{from_str, Value};
use std::collections::HashMap;

use crate::{
	fetch::{fetch, search_fetch},
	slice_on_last_dot, Class, Constructor, Enum, Error, Item, Result,
};

pub struct Graph {
	items: HashMap<String, Item>,
	qualified_names_by_url: HashMap<Url, String>,
}

impl Graph {
	/// Crate a new graph from the generated documentation at `docs_root` in the graph. `docs_root` must be a directory, having a trailing slash.
	pub async fn from_docs_root(docs_root: impl Into<Url>) -> Result<Graph> {
		let docs_root: Url = docs_root.into();
		let mut graph = Graph {
			items: HashMap::new(),
			qualified_names_by_url: HashMap::new(),
		};
		let package_name = get_name(docs_root.clone()).await?;
		let raw_items = parse_doc_index(
			fetch(docs_root.join("index.json").change_context(Error::HighlyOdd)?)
				.await?
				.ok_or(Error::InvalidDartDocOuput)?,
		)?;

		register_items(raw_items, package_name, &docs_root, &mut graph)?;

		Ok(graph)
	}

	/// Include the documentation for doc_item_url in the graph. Includes the entire package enclosing the particular item.
	///
	/// Normally, you'll want to try to call `Graph::get_qualified_name_from_url()` instead of this, as it will only scrape the docs for the item
	/// url if it is not already present in the graph
	pub async fn include_docs_item(&mut self, doc_item_url: impl Into<Url>) -> Result<()> {
		let (docs_root, text) = search_fetch(doc_item_url.into(), "index.json").await?.ok_or(Error::InvalidDartDocOuput)?;
		let raw_items = parse_doc_index(text)?;
		let package_name = get_name(docs_root.clone()).await?;

		register_items(raw_items, package_name, &docs_root, self)?;

		Ok(())
	}

	/// Add `item` to the graph. In most cases, you'll want to use `from_docs_root` instead, but this can be useful for defining builtins that
	/// will not be picked up by dartdoc.
	pub fn add_item(&mut self, qualified_name: String, item: Item) {
		let doc_url = match &item {
			Item::Class(Class { doc_url, .. }) => doc_url.clone(),
			Item::Enum(Enum { doc_url, .. }) => Some(doc_url.clone()),
		};

		self.items.insert(qualified_name.clone(), item);

		if let Some(doc_url) = doc_url {
			self.qualified_names_by_url.insert(doc_url, qualified_name);
		}
	}

	/// Mutably get an item by it's qualified name
	pub fn get_mut(&mut self, qualified_name: &str) -> Option<&mut Item> {
		self.items.get_mut(qualified_name)
	}

	/// Remove and return the item registered for a qualified name
	pub fn take(&mut self, qualified_name: &str) -> Option<Item> {
		let item = self.items.remove(qualified_name);

		if let Some(item) = &item {
			let doc_url = match item {
				Item::Class(Class { doc_url, .. }) => doc_url.as_ref(),
				Item::Enum(Enum { doc_url, .. }) => Some(doc_url),
			};

			if let Some(doc_url) = doc_url {
				self.qualified_names_by_url.remove(doc_url);
			}
		}

		item
	}

	/// Given a documentation url (for a class or enum, not a constructor), return the qualified name for the item, scraping the docs if it isn't
	/// present in the graph.
	pub async fn get_qualified_name_from_url(&mut self, url: &Url) -> Result<&str> {
		if self.qualified_names_by_url.contains_key(url) {
			Ok(self.qualified_names_by_url.get(url).unwrap().as_str())
		} else {
			self.include_docs_item(url.clone()).await?;

			Ok(match self.qualified_names_by_url.get(url) {
				Some(name) => name,
				None => Err(Error::UnknownItemUrl)?,
			})
		}
	}
}

fn register_items(doc_index: Vec<Value>, package_name: String, docs_root: &Url, graph: &mut Graph) -> Result<()> {
	for node in doc_index {
		let object = node.as_object().ok_or(Error::InvalidDartDocOuput)?;
		let kind = object
			.get("kind")
			.ok_or(Error::InvalidDartDocOuput)?
			.as_u64()
			.ok_or(Error::InvalidDartDocOuput)?;

		// dartdoc defines a "qualified name" as the name of the item appended to the library name
		// we take the one step further and prepend the package name also.
		//
		// This is why we call a dartdoc "qualified name" a "partial name".
		let partial_name = object
			.get("qualifiedName")
			.ok_or(Error::InvalidDartDocOuput)?
			.as_str()
			.ok_or(Error::InvalidDartDocOuput)?;

		let description = object
			.get("desc")
			.ok_or(Error::InvalidDartDocOuput)?
			.as_str() // PERF: could be optimized to take the owned string
			.ok_or(Error::InvalidDartDocOuput)?
			.to_string();
		let href = object
			.get("href")
			.ok_or(Error::InvalidDartDocOuput)?
			.as_str()
			.ok_or(Error::InvalidDartDocOuput)?;

		// Checks if this node is a ¿class? constructor (it could be an enum constructor)
		//
		// We search for constructor nodes instead of classes because it is the constructor that has all the
		// properties that will be need to construct the object
		//
		// Note: the qualified_name will often look like this "file.Class.Class", where the second instance of
		// "Class" is the name of the constructor, which is often the name of the class.
		//
		// If the class has no constructors, it can't be constructed and should therefore not be included in the
		// public api
		//
		// There may be multiple constructors for the same class, but that is ok. We will deduplicate this later
		//
		// This will also catch enum constructors (why is this a thing in dart smh), which are deduplicated a
		// few lines further down
		if kind == 2 {
			let (partial_name_option, constructor_name) = slice_on_last_dot(partial_name);
			let partial_name = partial_name_option
				.ok_or(Error::InvalidDartDocOuput)
				.attach_printable_lazy(|| "a fully qualified name should always have at least one dot in it")?;
			let class_qualified_name = format!("{package_name}.{partial_name}");
			let (_, class_name) = slice_on_last_dot(partial_name);

			let constructor = Constructor {
				name: constructor_name.to_string(),
				comment: description,
				properties: Vec::new(), // will be filled out later
				doc_url: docs_root.join(&href).change_context(Error::HighlyOdd)?,
			};

			if let Some(item) = graph.get_mut(&class_qualified_name) {
				let constructors = match item {
					Item::Class(Class { constructors, .. }) => constructors,
					Item::Enum { .. } => {
						// enums have constructors too (only one), and we don't want to register a constructors for them
						continue;
					}
				};

				constructors.push(constructor);
			} else {
				graph.add_item(
					partial_name.to_string(),
					Item::Class(Class {
						name: class_name.to_string(),
						comment: String::new(), // Value should be updated when the actual class is scraped
						constructors: Vec::from([constructor]),
						doc_url: None, // Value will be properly set when the class node is scraped
					}),
				);
			}
		}
		// Checks if this node is a class
		else if kind == 2 {
			let qualified_name = format!("{package_name}.{partial_name}");
			let doc_url = docs_root.join(&href).change_context(Error::HighlyOdd)?;

			if let Some(item) = graph.take(&qualified_name) {
				match item {
					Item::Class(Class { name, constructors, .. }) => graph.add_item(
						qualified_name,
						Item::Class(Class {
							name,
							comment: description,
							constructors,
							doc_url: Some(doc_url),
						}),
					),
					Item::Enum(_) => {
						warn!("found an enum at the qualified name that a class is supposed to go at. Probably the qualified names are borked");
					}
				}
			}
		}
		// Checks if this node is an enum
		else if kind == 5 {
			let qualified_name = format!("{package_name}.{partial_name}");
			let doc_url = docs_root.join(&href).change_context(Error::HighlyOdd)?;

			if let Some(item) = graph.take(&qualified_name) {
				match item {
					// if this item was already registered as a class, that was probably a false classification, due to enums all having
					// a default constructor.
					Item::Class(Class { name, .. }) => graph.add_item(
						qualified_name,
						Item::Enum(Enum {
							name,
							comment: description,
							variants: Vec::new(),
							doc_url,
						}),
					),
					Item::Enum { .. } => warn!("Tried to register an enum that was already registered. The qualified names are probably borked"),
				}
			} else {
				let name = slice_on_last_dot(&qualified_name).1.to_string();

				graph.add_item(
					qualified_name,
					Item::Enum(Enum {
						name,
						comment: description,
						variants: Vec::new(),
						doc_url,
					}),
				);
			}
		}
	}

	Ok(())
}

fn parse_doc_index(text: String) -> Result<Vec<Value>> {
	Ok(from_str(&text).change_context(Error::InvalidDartDocOuput)?)
}

/// Get the name of the package at `doc_dir`. Note: `doc_dir` should have a trailing slash
async fn get_name(doc_dir: Url) -> Result<String> {
	let url = doc_dir.join("index.html").change_context(Error::HighlyOdd)?;
	let dom = Dom::from_string(fetch(url).await?.ok_or(Error::InvalidDartDocOuput)?);

	let name = dom
		.query(&[Selector::Tag("header"), Selector::AnyChild, Selector::Tag("div"), Selector::Class("self-name")])
		.drain(..)
		.nth(0)
		.ok_or(Error::InvalidDartDocOuput)?
		.get_text();

	Ok(name)
}
