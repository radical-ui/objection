use error_stack::ResultExt;
use log::warn;
use reqwest::Url;
use serde_json::{from_str, Value};
use std::collections::HashMap;

use crate::{
	fetch::{fetch, search_fetch},
	html::HtmlPage,
	slice_on_last_dot, Error, Result,
};

pub struct DocIndex {
	items: Vec<Value>,
	docs_root: Url,
}

impl DocIndex {
	pub async fn from_docs_item(url: Url) -> Result<DocIndex> {
		let (docs_root, text) = search_fetch(url, "index.json").await?.ok_or(Error::InvalidDartDocOuput)?;
		let items = parse_doc_index(text)?;

		Ok(DocIndex { items, docs_root })
	}

	pub async fn from_docs_root(root_url: Url) -> Result<DocIndex> {
		let items = parse_doc_index(
			fetch(root_url.join("index.json").change_context(Error::HighlyOdd)?)
				.await?
				.ok_or(Error::InvalidDartDocOuput)?,
		)?;

		Ok(DocIndex { items, docs_root: root_url })
	}

	pub async fn get_items(&self) -> Result<HashMap<&str, DocItem<'_>>> {
		let mut roots = HashMap::<&str, DocItem<'_>>::new();

		for node in &self.items {
			let object = node.as_object().ok_or(Error::InvalidDartDocOuput)?;
			let kind = object
				.get("kind")
				.ok_or(Error::InvalidDartDocOuput)?
				.as_u64()
				.ok_or(Error::InvalidDartDocOuput)?;
			let qualified_name = object
				.get("qualifiedName")
				.ok_or(Error::InvalidDartDocOuput)?
				.as_str()
				.ok_or(Error::InvalidDartDocOuput)?;
			let description = object
				.get("desc")
				.ok_or(Error::InvalidDartDocOuput)?
				.as_str()
				.ok_or(Error::InvalidDartDocOuput)?;
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
				let (qualified_name_option, constructor_name) = slice_on_last_dot(qualified_name);
				let qualified_name = qualified_name_option
					.ok_or(Error::InvalidDartDocOuput)
					.attach_printable_lazy(|| "a fully qualified name should always have at least one dot in it")?;
				let (_, class_name) = slice_on_last_dot(qualified_name);

				let constructor = DocItemClassConstructor {
					name: constructor_name,
					page: HtmlPage::from_url(self.docs_root.join(href).change_context(Error::HighlyOdd)?)
						.await
						.change_context(Error::InvalidDartDocOuput)?,
				};

				if let Some(root) = roots.get_mut(qualified_name) {
					let constructors = match &mut root.kind {
						DocItemKind::Class { constructors } => constructors,
						DocItemKind::Enum { .. } => {
							// enums have constructors too (only one), and we don't want to register a constructors for them
							continue;
						}
					};

					constructors.push(constructor);
				} else {
					roots.insert(
						qualified_name,
						DocItem {
							name: class_name,
							description,
							kind: DocItemKind::Class {
								constructors: Vec::from([constructor]),
							},
						},
					);
				}
			}

			// Checks if this node is an enum
			if kind == 5 {
				let page = HtmlPage::from_url(self.docs_root.join(href).change_context(Error::HighlyOdd)?)
					.await
					.change_context(Error::InvalidDartDocOuput)?;

				if let Some(root) = roots.get_mut(qualified_name) {
					match root.kind {
						// if this item was already registered as a class, that was probably a false classification, due to enums all having
						// a default constructor.
						DocItemKind::Class { .. } => root.kind = DocItemKind::Enum { page },
						DocItemKind::Enum { .. } => warn!("Tried to register an enum that was already registered. The qualified names are probably borked"),
					}
				} else {
					roots.insert(
						qualified_name,
						DocItem {
							name: slice_on_last_dot(qualified_name).1,
							description,
							kind: DocItemKind::Enum { page },
						},
					);
				}
			}
		}

		Ok(roots)
	}
}

pub struct DocItem<'a> {
	pub name: &'a str,
	pub description: &'a str,
	pub kind: DocItemKind<'a>,
}

pub enum DocItemKind<'a> {
	Class { constructors: Vec<DocItemClassConstructor<'a>> },
	Enum { page: HtmlPage },
}

pub struct DocItemClassConstructor<'a> {
	name: &'a str,
	page: HtmlPage,
}

fn parse_doc_index(text: String) -> Result<Vec<Value>> {
	Ok(from_str(&text).change_context(Error::InvalidDartDocOuput)?)
}
