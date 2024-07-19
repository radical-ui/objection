use anyhow::{anyhow, Context, Error, Result};
use deno_doc::{interface::InterfaceDef, js_doc::JsDocTag, ts_type::TsTypeDef, type_alias, DocNodeKind, DocParser, DocParserOptions, Location};
use deno_graph::{source::MemoryLoader, BuildOptions, CapturingModuleAnalyzer, GraphKind, ModuleGraph};
use log::{debug, warn};
use std::collections::{HashMap, HashSet};
use url::Url;

use crate::contextual_format;

#[derive(Debug)]
pub enum Kind {
	Dynamic,
	String,
	Number,
	Bool,
	Null,
	ActionKey { data_type: Box<Kind> },
	EventKey { data_type: Box<Kind> },
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
	pub render_name: String,
	/// The action name, linked to the type of the action data
	pub actions: HashMap<String, String>,
	/// The event name, linked to the type of the event data
	pub events: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct Collection {
	action_key_type_name: Option<String>,
	event_key_type_name: Option<String>,
	kinds: HashMap<String, InternalKindDefinition>,
	erroring_kinds: HashMap<String, Error>,
	components: HashMap<String, ComponentInfo>,
	functions: HashSet<String>,
	erroring_functions: HashMap<String, Error>,
}

impl Collection {
	pub async fn collect(&mut self, runtime_url: &Url, memory_loader: &MemoryLoader) -> Result<()> {
		let analyzer = CapturingModuleAnalyzer::default();
		let mut graph = ModuleGraph::new(GraphKind::TypesOnly);

		let diagnostics = graph
			.build(
				Vec::from([runtime_url.clone()]),
				memory_loader,
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

		let nodes = parser.parse_with_reexports(runtime_url)?;

		for node in &nodes {
			self.consider_js_doc_tags(&node.name, &node.js_doc.tags)
		}

		if let None = &self.event_key_type_name {
			warn!(
				"{}",
				contextual_format(
					"No type was found for noting event keys",
					"Runtime events will not be recognized without a @feature_event_key js doc type tag to notate them. Additionally, this type must be exported from the runtime."
				)
			);
		}

		if let None = &self.action_key_type_name {
			warn!(
				"{}",
				contextual_format(
					"No type was found for noting action keys",
					"Runtime action types will not be recognized without a @feature_action_key js doc tag to notate them. Additionally, this type must be exported from the runtime."
				)
			);
		}

		for node in nodes {
			let name = node.name.clone();

			match node.kind {
				DocNodeKind::Function => {
					self.functions.insert(name);
				}
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
					let conversion = convert_interface(ConvertInterfaceParams {
						interface: node.interface_def.as_ref().ok_or(anyhow!("Bad deno_doc output: expected interface def."))?,
						location: &node.location,
						component: self.components.get_mut(&name),
						action_key_type_name: self.action_key_type_name.as_deref(),
						event_key_type_name: self.event_key_type_name.as_deref(),
					});

					match conversion {
						Ok(Conversion { kind, dependencies }) => {
							self.kinds.insert(
								name,
								InternalKindDefinition {
									comment: node.js_doc.doc.clone(),
									kind,
									dependencies,
								},
							);
						}
						Err(error) => {
							self.erroring_kinds.insert(
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
						self.erroring_kinds
							.insert(name, local_error("Type parameters are not supported.", &node.location));
					} else {
						let conversion = convert_ts_type(ConvertTsTypeParams {
							ts_type: &type_alias.ts_type,
							location: &node.location,
							component: self.components.get_mut(&node.name),
							action_key_type_name: self.action_key_type_name.as_deref(),
							event_key_type_name: self.event_key_type_name.as_deref(),
						});

						match conversion {
							Ok(Conversion { kind, dependencies }) => {
								self.kinds.insert(
									name,
									InternalKindDefinition {
										comment: node.js_doc.doc.clone(),
										kind,
										dependencies,
									},
								);
							}
							Err(error) => {
								self.erroring_kinds
									.insert(name, error.context(local_message("Failed to convert type alias.", &node.location)));
							}
						};
					}
				}
				DocNodeKind::Variable => local_warn(
					"Exported variables are not supported and will be ignored. If you want to export a component render \
					function, `export function` instead",
					&node.location,
				),
			}
		}

		Ok(())
	}

	pub fn check_components(&mut self) {
		let components = self.get_component_info().iter().map(|(name, _)| *name).collect::<Vec<_>>();
		let unreachable_names = self.get_unrelated_names(components).iter().map(|name| name.to_string()).collect::<Vec<_>>();

		self.prune_names(unreachable_names.iter().map(|item| item.as_str()));
		self.meet_all_dependencies();

		if !self.functions.contains("start") {
			self.erroring_functions.insert(
				"start".to_string(),
				anyhow!(
					"{}",
					&contextual_format("Missing function `start`", "All renderers must export a `start` function.")
				),
			);
		}

		for (name, component) in &self.components {
			if !self.functions.contains(&component.render_name) {
				self.erroring_functions.insert(
					component.render_name.clone(),
					anyhow!(
						"{}",
						contextual_format(
							&format!("Missing function `{}`", &component.render_name),
							&format!("Specified as the renderer for `{}`, but it was not exported", &name)
						)
					),
				);
			}
		}
	}

	pub fn get_component_info(&self) -> Vec<(&str, &ComponentInfo)> {
		self.components.iter().map(|(name, info)| (name.as_str(), info)).collect()
	}

	pub fn get_all_names(&self) -> Vec<&str> {
		let mut keys = self.kinds.keys().map(|s| s.as_str()).collect::<Vec<_>>();

		for item in self.erroring_kinds.keys() {
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
			remove_dependencies(name, &self.kinds, &mut marked_nodes);
		}

		marked_nodes.drain().collect()
	}

	pub fn prune_names<'a>(&mut self, names: impl IntoIterator<Item = &'a str>) {
		for name in names {
			self.kinds.remove(name);
			self.erroring_kinds.remove(name);
		}
	}

	pub fn meet_all_dependencies(&mut self) {
		let mut missing = HashMap::<String, Vec<String>>::new();

		for (name, InternalKindDefinition { dependencies, .. }) in &self.kinds {
			for dependency in dependencies {
				if self.kinds.contains_key(dependency) || self.erroring_kinds.contains_key(dependency) {
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
			self.erroring_kinds.insert(
				name.clone(),
				anyhow!(
					"{}",
					contextual_format(
						"Missing type `{name}`",
						&format!(
							"Expected because it was referenced by {}",
							dependents.iter().map(|d| format!("`{d}`")).collect::<Vec<_>>().join(", ")
						)
					),
				),
			);
		}
	}

	pub fn get_errors(&self) -> Vec<&Error> {
		let mut kind_errors = self.erroring_kinds.values().collect::<Vec<_>>();

		for error in self.erroring_functions.values() {
			kind_errors.push(error);
		}

		kind_errors
	}

	pub fn get_kinds(&self) -> Vec<KindDefinition> {
		self.kinds
			.iter()
			.map(|(name, def)| KindDefinition {
				name,
				comment: def.comment.as_deref(),
				kind: &def.kind,
			})
			.collect()
	}

	fn consider_js_doc_tags(&mut self, node_name: &str, tags: &[JsDocTag]) {
		for tag in tags {
			if let JsDocTag::Unsupported { value } = tag {
				let mut words = value.split_whitespace().rev().collect::<Vec<_>>();
				let label = words.pop().unwrap();
				let context = words.pop();

				if label == "@component" {
					let render_name = context.map(|inner| inner.to_string()).unwrap_or(format!("{node_name}Render"));

					self.components.insert(
						node_name.to_string(),
						ComponentInfo {
							render_name,
							actions: HashMap::new(),
							events: HashMap::new(),
						},
					);
				} else if value == "@feature_event_key" {
					self.event_key_type_name = Some(node_name.to_string());
				} else if value == "@feature_action_key" {
					self.action_key_type_name = Some(node_name.to_string());
				}
			}
		}
	}
}

struct Conversion {
	kind: Kind,
	dependencies: Vec<String>,
}

struct ConvertInterfaceParams<'a> {
	interface: &'a InterfaceDef,
	location: &'a Location,
	component: Option<&'a mut ComponentInfo>,
	action_key_type_name: Option<&'a str>,
	event_key_type_name: Option<&'a str>,
}

fn convert_interface(params: ConvertInterfaceParams<'_>) -> Result<Conversion> {
	let ConvertInterfaceParams {
		interface,
		location,
		mut component,
		action_key_type_name,
		event_key_type_name,
	} = params;

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

		let mut conversion = convert_ts_type(ConvertTsTypeParams {
			ts_type: type_def,
			location: &property_def.location,
			component: component.as_deref_mut(),
			action_key_type_name,
			event_key_type_name,
		})
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

struct ConvertTsTypeParams<'a> {
	ts_type: &'a TsTypeDef,
	location: &'a Location,
	component: Option<&'a mut ComponentInfo>,
	action_key_type_name: Option<&'a str>,
	event_key_type_name: Option<&'a str>,
}

fn convert_ts_type(params: ConvertTsTypeParams<'_>) -> Result<Conversion> {
	let ConvertTsTypeParams {
		ts_type,
		location,
		mut component,
		action_key_type_name,
		event_key_type_name,
	} = params;

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

		if keyword == "null" {
			return Ok(Conversion {
				kind: Kind::Null,
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

	if let Some(type_ref) = &params.ts_type.type_ref {
		let mut type_params = Vec::with_capacity(type_ref.type_params.as_ref().map(|params| params.len()).unwrap_or_default());

		if let Some(ts_type_params) = &type_ref.type_params {
			for ts_type in ts_type_params {
				type_params.push(convert_ts_type(ConvertTsTypeParams {
					ts_type,
					location,
					component: component.as_deref_mut(),
					action_key_type_name,
					event_key_type_name,
				})?);
			}
		}

		if let Some(action_key_type_name) = action_key_type_name {
			if &type_ref.type_name == action_key_type_name {
				if type_params.len() != 1 {
					return local_err(
						&format!(
							"Because it is an action key, expected to find 1 type parameter for `{action_key_type_name}`, but found {}",
							type_params.len()
						),
						location,
					);
				}

				let data_type = type_params.swap_remove(0);

				return Ok(Conversion {
					kind: Kind::ActionKey {
						data_type: Box::new(data_type.kind),
					},
					dependencies: data_type.dependencies,
				});
			}
		}

		if let Some(event_key_type_name) = event_key_type_name {
			if &type_ref.type_name == event_key_type_name {
				if type_params.len() != 1 {
					return local_err(
						&format!(
							"Because it is an event key, expected to find 1 type parameter for `{event_key_type_name}`, but found {}",
							type_params.len()
						),
						location,
					);
				}

				let data_type = type_params.swap_remove(0);

				return Ok(Conversion {
					kind: Kind::EventKey {
						data_type: Box::new(data_type.kind),
					},
					dependencies: data_type.dependencies,
				});
			}
		}

		if type_ref.type_params.is_some() {
			return local_err(
				&format!("Type `{}` was supplied type parameters, but this is not supported", &type_ref.type_name),
				location,
			);
		}

		return Ok(Conversion {
			kind: Kind::Ref {
				name: type_ref.type_name.clone(),
			},
			dependencies: Vec::new(),
		});
	}

	if let Some(array) = &ts_type.array {
		let inner_conversion = convert_ts_type(ConvertTsTypeParams {
			ts_type: array,
			location,
			component,
			action_key_type_name,
			event_key_type_name,
		})?;

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
			let mut inner_conversion = convert_ts_type(ConvertTsTypeParams {
				ts_type: ts_item,
				location,
				component: component.as_deref_mut(),
				action_key_type_name,
				event_key_type_name,
			})
			.context("Failed to convert tuple")?;

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
						let mut def_conversion = convert_ts_type(ConvertTsTypeParams {
							ts_type: property
								.ts_type
								.as_ref()
								.ok_or(local_error("Expected to find a type assiciated with the 'def' field", &property.location))?,
							location: &property.location,
							component: component.as_deref_mut(),
							action_key_type_name,
							event_key_type_name,
						})
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

	debug!("Encountered an unknown type: {:#?}", ts_type);
	local_err("Unsupported type", location)
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
	let file = format!("{}:{}:{}", location.filename, location.line, location.col);

	contextual_format(message, &file)
}
