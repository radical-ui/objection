use anyhow::{anyhow, Context, Error, Result};
use colored::Colorize;
use deno_doc::{interface::InterfaceDef, js_doc::JsDocTag, ts_type::TsTypeDef, DocNodeKind, DocParser, DocParserOptions, Location};
use deno_graph::{BuildOptions, CapturingModuleAnalyzer, GraphKind, ModuleGraph};
use log::{debug, warn};
use std::collections::{HashMap, HashSet};
use url::Url;

use crate::module_loader::load_modules;

#[derive(Debug)]
pub enum Kind {
	Dynamic,
	String,
	Number,
	Bool,
	Component,
	Ref { name: String },
	List { of: Box<Kind> },
	Tuple { items: Vec<Kind> },
	StringEnum { variants: Vec<String> },
	KeyedEnum { variants: Vec<Property> },
	Object { properties: Vec<Property> },
}

#[derive(Debug)]
pub struct KindDefinition<'a> {
	pub name: &'a str,
	pub comment: Option<&'a str>,
	pub kind: &'a Kind,
}

#[derive(Debug)]
struct InternalKindDefinition {
	pub comment: Option<String>,
	pub kind: Kind,
	pub dependencies: Vec<String>,
}

#[derive(Debug)]
pub struct Property {
	pub comment: Option<String>,
	pub name: String,
	pub kind: Kind,
}

#[derive(Debug)]
pub struct ComponentInfo {
	kind_name: String,
	render_name: String,
}

#[derive(Debug, Default)]
pub struct Collection {
	conversions: HashMap<String, InternalKindDefinition>,
	failed_conversions: HashMap<String, Error>,
	components: Vec<ComponentInfo>,
	function_names: Vec<String>,
}

impl Collection {
	pub async fn collect(&mut self, runtime_url: &Url) -> Result<()> {
		let loader = load_modules(runtime_url).await?;
		let analyzer = CapturingModuleAnalyzer::default();
		let mut graph = ModuleGraph::new(GraphKind::TypesOnly);

		let diagnostics = graph
			.build(
				Vec::from([runtime_url.clone()]),
				&loader,
				BuildOptions {
					module_analyzer: &analyzer,
					..Default::default()
				},
			)
			.await;

		for diagnostic in diagnostics {
			println!("{}", diagnostic);
		}

		let parser = DocParser::new(
			&graph,
			&analyzer,
			DocParserOptions {
				diagnostics: true,
				private: false,
			},
		)?;

		for node in parser.parse_with_reexports(runtime_url)? {
			let name = node.name.clone();

			match node.kind {
				DocNodeKind::Function => self.function_names.push(name),
				DocNodeKind::Class => local_warn("Classes are not a support type of export and will be ignored (at {})", &node.location),
				DocNodeKind::Enum => local_warn(
					"Enums are not a supported type of export and will be ignored. Use a keyed or string literal union instead (at {})",
					&node.location,
				),
				DocNodeKind::Import => (), // TODO we should figure out how to handle the "import item as anotherItem" cases
				DocNodeKind::ModuleDoc => local_warn(
					"Module docs are ignored. To document a specific component, place the doc comment on that component's interface (at {})",
					&node.location,
				),
				DocNodeKind::Interface => {
					let conversion = convert_interface(
						node.interface_def.as_ref().ok_or(anyhow!("Bad deno_doc output: expected interface def."))?,
						&node.location,
					);

					self.consider_js_doc_tags(&name, node.js_doc.tags);

					match conversion {
						Ok(Conversion { kind, dependencies }) => {
							self.conversions.insert(
								name,
								InternalKindDefinition {
									comment: node.js_doc.doc.clone(),
									kind,
									dependencies,
								},
							);
						}
						Err(error) => {
							self.failed_conversions.insert(
								name,
								error.context(local_message(&format!("Failed to convert interface `{}`", &node.name), &node.location)),
							);
						}
					};
				}
				DocNodeKind::Namespace => local_warn("Namespaces are not supported and will be ignored (at {})", &node.location),
				DocNodeKind::TypeAlias => {
					let type_alias = node
						.type_alias_def
						.as_ref()
						.ok_or(anyhow!("Bad deno_doc output: expected type alias def for node of kind type alias."))?;

					if !type_alias.type_params.is_empty() {
						self.failed_conversions
							.insert(name, local_error("Type parameters are not supported.", &node.location));
					} else {
						let conversion = convert_ts_type(&type_alias.ts_type, &node.location);

						self.consider_js_doc_tags(&name, node.js_doc.tags);

						match conversion {
							Ok(Conversion { kind, dependencies }) => {
								self.conversions.insert(
									name,
									InternalKindDefinition {
										comment: node.js_doc.doc.clone(),
										kind,
										dependencies,
									},
								);
							}
							Err(error) => {
								self.failed_conversions
									.insert(name, error.context(local_message("Failed to convert type alias.", &node.location)));
							}
						};
					}
				}
				DocNodeKind::Variable => local_warn(
					"Exported variables are not supported and will be ignored. If you want to export a component render \
						function, `export function` instead (at {})",
					&node.location,
				),
			}
		}

		Ok(())
	}

	pub fn check_components(&mut self) {
		let components = self.get_component_info().iter().map(|info| info.kind_name.as_str()).collect::<Vec<_>>();
		let unreachable_names = self.get_unrelated_names(components).iter().map(|name| name.to_string()).collect::<Vec<_>>();

		self.prune_names(unreachable_names.iter().map(|item| item.as_str()));
		self.meet_all_dependencies();
	}

	pub fn get_component_info(&self) -> &[ComponentInfo] {
		&self.components
	}

	pub fn get_all_names(&self) -> Vec<&str> {
		let mut keys = self.conversions.keys().map(|s| s.as_str()).collect::<Vec<_>>();

		for item in self.failed_conversions.keys() {
			keys.push(item.as_str());
		}

		keys
	}

	pub fn get_unrelated_names<'a>(&'a self, names: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
		let mut marked_nodes = HashSet::<&'a str>::from_iter(self.get_all_names());

		fn remove_dependencies(name: &str, conversions: &HashMap<String, InternalKindDefinition>, marked_nodes: &mut HashSet<&str>) {
			if let Some(definition) = conversions.get(name) {
				for dependency in &definition.dependencies {
					marked_nodes.remove(dependency.as_str());
					remove_dependencies(name, conversions, marked_nodes);
				}
			}
		}

		for name in names {
			marked_nodes.remove(name);
			remove_dependencies(name, &self.conversions, &mut marked_nodes);
		}

		marked_nodes.drain().collect()
	}

	pub fn prune_names<'a>(&mut self, names: impl IntoIterator<Item = &'a str>) {
		for name in names {
			self.conversions.remove(name);
			self.failed_conversions.remove(name);
		}
	}

	pub fn meet_all_dependencies(&mut self) {
		let mut missing = HashMap::<String, Vec<String>>::new();

		for (name, InternalKindDefinition { dependencies, .. }) in &self.conversions {
			for dependency in dependencies {
				if self.conversions.contains_key(dependency) || self.failed_conversions.contains_key(dependency) {
					continue;
				}

				if let Some(dependents) = missing.get_mut(dependency) {
					dependents.push(name.clone());
				} else {
					missing.insert(dependency.clone(), Vec::from([name.clone()]));
				}
			}
		}

		for (name, dependents) in missing {
			self.failed_conversions.insert(
				name.clone(),
				anyhow!(
					"Couldn't find an exported item for `{name}`, referenced by {}",
					dependents.iter().map(|d| format!("`{d}`")).collect::<Vec<_>>().join(", ")
				),
			);
		}
	}

	pub fn get_errors(&self) -> Vec<&Error> {
		self.failed_conversions.values().collect()
	}

	pub fn get_kinds(&self) -> Vec<KindDefinition> {
		self.conversions
			.iter()
			.map(|(name, def)| KindDefinition {
				name,
				comment: def.comment.as_deref(),
				kind: &def.kind,
			})
			.collect()
	}

	fn consider_js_doc_tags(&mut self, node_name: &str, tags: Vec<JsDocTag>) {
		for tag in tags {
			if let JsDocTag::Unsupported { value } = tag {
				let mut words = value.split_whitespace().rev().collect::<Vec<_>>();
				let label = words.pop().unwrap();
				let context = words.pop();

				if label == "@component" {
					let render_name = context.map(|inner| inner.to_string()).unwrap_or(format!("Render{node_name}"));

					self.components.push(ComponentInfo {
						kind_name: node_name.to_string(),
						render_name,
					});
				}
			}
		}
	}
}

struct Conversion {
	kind: Kind,
	dependencies: Vec<String>,
}

fn convert_interface(interface: &InterfaceDef, location: &Location) -> Result<Conversion> {
	let mut interface_dependencies = Vec::new();
	let mut properties = Vec::new();

	if !interface.extends.is_empty() {
		return local_err(
			"Interface extensions are not supported, instead specify all properties in the interface body",
			location,
		);
	}

	if !interface.methods.is_empty() {
		return local_err(
			"Methods are not supported in exported interfaces. If this is releated to private client-only functionality, consider \
			inlining the methods in the render function (at {})",
			location,
		);
	}

	for property_def in &interface.properties {
		let type_def = match &property_def.ts_type {
			Some(def) => def,
			None => {
				return local_err("Interface property does not have an associated type (at {})", &property_def.location);
			}
		};

		let mut conversion = convert_ts_type(type_def, &property_def.location)
			.with_context(|| local_message(&format!("Failed to convert interface property `{}`", property_def.name), &property_def.location))?;

		interface_dependencies.append(&mut conversion.dependencies);

		properties.push(Property {
			comment: property_def.js_doc.doc.clone(),
			name: property_def.name.to_string(),
			kind: conversion.kind,
		})
	}

	Ok(Conversion {
		kind: Kind::Object { properties },
		dependencies: interface_dependencies,
	})
}

fn convert_ts_type(ts_type: &TsTypeDef, location: &Location) -> Result<Conversion> {
	if let Some(keyword) = &ts_type.keyword {
		if keyword == "string" {
			return Ok(Conversion {
				kind: Kind::String,
				dependencies: Vec::new(),
			});
		}

		if keyword == "number" {
			return Ok(Conversion {
				kind: Kind::Number,
				dependencies: Vec::new(),
			});
		}

		if keyword == "boolean" {
			return Ok(Conversion {
				kind: Kind::Bool,
				dependencies: Vec::new(),
			});
		}

		if keyword == "unknown" {
			return Ok(Conversion {
				kind: Kind::Dynamic,
				dependencies: Vec::new(),
			});
		}

		if keyword == "any" {
			return local_err("Use 'unknown' instead of 'any'", location);
		}

		return local_err(&format!("Unknown keyword '{keyword}'"), location);
	}

	if let Some(type_ref) = &ts_type.type_ref {
		if type_ref.type_name == "Component" {
			return Ok(Conversion {
				kind: Kind::Component,
				dependencies: Vec::new(),
			});
		}

		if type_ref.type_params.is_some() {
			local_warn("Type ref was supplied parameters, but this is not supported", location)
		}

		return Ok(Conversion {
			kind: Kind::Ref {
				name: type_ref.type_name.clone(),
			},
			dependencies: Vec::new(),
		});
	}

	if let Some(array) = &ts_type.array {
		let inner_conversion = convert_ts_type(array, location)?;

		return Ok(Conversion {
			kind: Kind::List {
				of: Box::new(inner_conversion.kind),
			},
			dependencies: inner_conversion.dependencies,
		});
	}

	if let Some(tuple) = &ts_type.tuple {
		let mut combined_dependencies = Vec::new();
		let mut items = Vec::new();

		for ts_item in tuple {
			let mut inner_conversion = convert_ts_type(ts_item, location).context("Failed to convert tuple")?;

			items.push(inner_conversion.kind);
			combined_dependencies.append(&mut inner_conversion.dependencies);
		}

		return Ok(Conversion {
			kind: Kind::Tuple { items },
			dependencies: combined_dependencies,
		});
	}

	if let Some(union) = &ts_type.union {
		let mut combined_dependencies = Vec::new();
		let mut string_variants = Vec::new();
		let mut keyed_variants = Vec::new();
		let mut variant_number = 1;

		for ts_type in union {
			if let Some(literal) = &ts_type.literal {
				if let Some(string) = &literal.string {
					string_variants.push(string.clone());
				} else {
					return local_err(
						&format!("Failed to convert variant #{variant_number} in union. Only string literals and keyed objects are supported."),
						location,
					);
				}
			} else if let Some(type_literal) = &ts_type.type_literal {
				let mut comment = None;
				let mut name = None;
				let mut definition_kind = None;

				for property in &type_literal.properties {
					if &property.name == "type" {
						comment = property.js_doc.doc.clone();

						let type_type = property
							.ts_type
							.as_ref()
							.ok_or(local_error("Expected to find a type associated with the 'type' field", &property.location))?;

						if let Some(literal) = &type_type.literal {
							if let Some(string) = &literal.string {
								name = Some(string.clone())
							} else {
								return local_err(
									"The type of the 'type' property must be a string literal, as this is a keyed object",
									&property.location,
								);
							}
						} else {
							return local_err(
								"The type of the 'type' property must be a string literal, as this is a keyed object",
								&property.location,
							);
						}
					} else if &property.name == "def" {
						let mut def_conversion = convert_ts_type(
							property
								.ts_type
								.as_ref()
								.ok_or(local_error("Expected to find a type assiciated with the 'def' field", &property.location))?,
							&property.location,
						)
						.with_context(|| local_message(&format!("Failed to convert property {}", &property.name), &property.location))?;

						combined_dependencies.append(&mut def_conversion.dependencies);
						definition_kind = Some(def_conversion.kind);
					}
				}

				let name = name.ok_or(local_error(
					"Union variant #{variant_number} is not a valid keyed object. No 'type' field was found.",
					location,
				))?;
				let kind = definition_kind.ok_or(local_error(
					"Union variant #{variant_number} is not a valid keyed object. No 'def' field was found.",
					location,
				))?;

				keyed_variants.push(Property { comment, name, kind });
			} else {
				return local_err(
					"Unsupported enum type in variant #{variant_number}. Only string literals and keyed objects are supported.",
					location,
				);
			}

			variant_number += 1;
		}

		if !string_variants.is_empty() && !keyed_variants.is_empty() {
			return local_err("Found a union with both string and keyed object variants. This is not allowed. The entire union must be made up of either string literals or keyed objects", location);
		}

		return Ok(if !string_variants.is_empty() {
			Conversion {
				kind: Kind::StringEnum { variants: string_variants },
				dependencies: Vec::new(),
			}
		} else {
			Conversion {
				kind: Kind::KeyedEnum { variants: keyed_variants },
				dependencies: combined_dependencies,
			}
		});
	}

	local_warn("Unknown type", location);
	debug!("Type: {:#?}", ts_type);

	Ok(Conversion {
		kind: Kind::Bool,
		dependencies: Vec::new(),
	})
}

fn local_warn(message: &str, location: &Location) {
	warn!("{:?}", local_error(message, location))
}

fn local_error(message: &str, location: &Location) -> Error {
	anyhow!("{}", local_message(message, location))
}

fn local_err<T>(message: &str, location: &Location) -> Result<T> {
	Err(local_error(message, location))
}

fn local_message(message: &str, location: &Location) -> String {
	let file = format!("at {}:{}:{}", location.filename, location.line, location.col);

	format!("{message}\n    {}", file.dimmed())
}
