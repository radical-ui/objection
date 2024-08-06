use anyhow::{anyhow, Error, Result};
use deno_doc::{js_doc::JsDocTag, DocNodeKind, DocParser, DocParserOptions};
use deno_graph::{source::MemoryLoader, BuildOptions, CapturingModuleAnalyzer, GraphKind, ModuleGraph};
use log::{debug, trace};
use std::collections::{HashMap, HashSet};
use url::Url;

use crate::{
	convert::{convert_interface, convert_ts_type, Conversion, ConvertInterfaceParams, ConvertTsTypeParams, Kind},
	diagnostic::Diagnostic,
};

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
pub struct ComponentInfo {
	/// The name that the renderer has chosen to use to refer to the function that renders this component
	pub render_name: String,
}

#[derive(Debug, Default)]
pub struct Collection {
	action_key_type_name: Option<String>,
	event_key_type_name: Option<String>,
	component_index_name: Option<String>,
	kinds: HashMap<String, InternalKindDefinition>,
	erroring_kinds: HashMap<String, Error>,
	components: HashMap<String, ComponentInfo>,
	functions: HashSet<String>,
	erroring_functions: HashMap<String, Error>,
	other_diagnostics: Vec<Error>,
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
			Diagnostic::start("No type was found for noting event keys")
				.shift()
				.text("Runtime events will not be recognized without a @feature_event_key js doc type tag to notate them. Additionally, this type must be exported from the runtime.")
				.build()
				.print_warn();
		}

		if let None = &self.action_key_type_name {
			Diagnostic::start("No type was found for noting action keys")
				.shift()
				.text("Runtime action types will not be recognized without a @feature_action_key js doc tag to notate them. Additionally, this type must be exported from the runtime.").build()
				.print_warn();
		}

		for node in nodes {
			let name = node.name.clone();

			match node.kind {
				DocNodeKind::Function => {
					self.functions.insert(name);
				}
				DocNodeKind::Class => Diagnostic::start("Classes are not a supported type of export and will be ignored")
					.shift()
					.location(&node.location)
					.build()
					.print_warn(),
				DocNodeKind::Enum => {
					Diagnostic::start("Enums are not a supported type of export and will be ignored. Use a keyed or string literal union instead")
						.shift()
						.location(&node.location)
						.build()
						.print_warn()
				}
				DocNodeKind::Import => (), // TODO we should figure out how to handle the "import item as anotherItem" cases
				DocNodeKind::ModuleDoc => {
					Diagnostic::start("Module docs are ignored. To document a specific component, place the doc comment on that component's interface")
						.shift()
						.location(&node.location)
						.build()
						.print_warn()
				}
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
								error.context(
									Diagnostic::start("Failed to convert interface ")
										.inline_code(&node.name)
										.shift()
										.location(&node.location)
										.build(),
								),
							);
						}
					};
				}
				DocNodeKind::Namespace => Diagnostic::start("Namespaces are not supported and will be ignored")
					.shift()
					.location(&node.location)
					.build()
					.print_warn(),
				DocNodeKind::TypeAlias => {
					let type_alias = node
						.type_alias_def
						.as_ref()
						.ok_or(anyhow!("Bad deno_doc output: expected type alias def for node of kind type alias."))?;

					if !type_alias.type_params.is_empty() {
						self.erroring_kinds.insert(
							name,
							Diagnostic::start("Type parameters are not supported")
								.shift()
								.location(&node.location)
								.build()
								.error(),
						);
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
								self.erroring_kinds.insert(
									name.clone(),
									error.context(
										Diagnostic::start("Failed to convert type alias ")
											.inline_code(&name)
											.shift()
											.location(&node.location)
											.build()
											.error(),
									),
								);
							}
						};
					}
				}
				DocNodeKind::Variable => Diagnostic::start(
					"Exported variables are not supported and will be ignored. If you want to export a component render \
					function, `export function` instead",
				)
				.shift()
				.location(&node.location)
				.build()
				.print_error(),
			}
		}

		Ok(())
	}

	pub fn get_component_index_name(&self) -> Option<&str> {
		self.component_index_name.as_deref()
	}

	pub fn check_components(&mut self) {
		// TODO all of this should be in `Inspect`

		let components = self.get_component_info().iter().map(|(name, _)| *name).collect::<Vec<_>>();
		let unreachable_names = self.get_unrelated_names(components).iter().map(|name| name.to_string()).collect::<Vec<_>>();

		debug!("Removing the following names from the graph because they were deemed unreachable by component types: {unreachable_names:#?}");

		self.prune_names(unreachable_names.iter().map(|item| item.as_str()));
		self.meet_all_dependencies();

		if !self.functions.contains("start") {
			self.erroring_functions.insert(
				"start".to_string(),
				Diagnostic::start("Missing function")
					.inline_code("start")
					.shift()
					.text("All renderers must export a")
					.inline_code("start")
					.text("function")
					.build()
					.error(),
			);
		}

		for (name, component) in &self.components {
			if !self.functions.contains(&component.render_name) {
				self.erroring_functions.insert(
					component.render_name.clone(),
					Diagnostic::start("Missing function ")
						.inline_code(&component.render_name)
						.shift()
						.text("Specified as the renderer for ")
						.inline_code(&name)
						.text(", but it was not exported")
						.build()
						.error(),
				);
			}
		}

		if let Some(name) = &self.component_index_name {
			debug!("Found `{name}` as the component index");
		} else {
			self.other_diagnostics.push(
				Diagnostic::start("No component index was specified")
					.shift()
					.text("Either annotate a component with @feature_component_index, or export ")
					.inline_code("Component")
					.text(" provided by runtime_lib")
					.build()
					.error(),
			)
		}
	}

	pub fn get_comment(&self, kind_name: &str) -> Option<&str> {
		self.kinds.get(kind_name).map(|item| item.comment.as_deref()).flatten()
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

	pub fn resolve_kind<'a>(&'a self, kind: &'a Kind) -> (&'a Kind, Option<&str>) {
		if let Kind::Ref { name } = kind {
			if let Some(backing) = self.kinds.get(name) {
				let (kind, resolved_name) = self.resolve_kind(&backing.kind);

				(kind, Some(resolved_name.unwrap_or(name.as_str())))
			} else {
				(kind, None)
			}
		} else {
			(kind, None)
		}
	}

	pub fn get_unrelated_names<'a>(&'a self, names: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
		let mut marked_nodes = HashSet::<&'a str>::new();

		fn mark_dependencies<'b>(name: &str, kinds: &'b HashMap<String, InternalKindDefinition>, marked_nodes: &mut HashSet<&'b str>) {
			if let Some(def) = kinds.get(name) {
				for dependency in &def.dependencies {
					if !marked_nodes.contains(dependency.as_str()) {
						trace!("marking dependency: {name} => {dependency}");

						marked_nodes.insert(dependency.as_str());
						mark_dependencies(dependency.as_str(), kinds, marked_nodes);
					}
				}
			}
		}

		for name in names {
			marked_nodes.insert(name);
			mark_dependencies(name, &self.kinds, &mut marked_nodes);
		}

		self.kinds
			.keys()
			.map(|key| key.as_str())
			.filter(|key| !marked_nodes.contains(key))
			.chain(self.erroring_kinds.keys().map(|key| key.as_str()).filter(|key| !marked_nodes.contains(key)))
			.collect()
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
				Diagnostic::start("Missing type ")
					.inline_code(&name)
					.shift()
					.text("Expected because it was referenced by ")
					.join_map(dependents.iter(), |builder, dependent| builder.inline_code(&dependent))
					.build()
					.error(),
			);
		}
	}

	pub fn get_errors(&self) -> Vec<&Error> {
		let kind_errors = self
			.erroring_kinds
			.values()
			.chain(self.erroring_functions.values())
			.chain(self.other_diagnostics.iter())
			.collect::<Vec<_>>();

		kind_errors
	}

	pub fn get_kinds(&self) -> Vec<KindDefinition> {
		let mut kinds = self
			.kinds
			.iter()
			.map(|(name, def)| KindDefinition {
				name,
				comment: def.comment.as_deref(),
				kind: &def.kind,
			})
			.collect::<Vec<_>>();

		kinds.sort_by(|a, b| a.name.cmp(b.name));

		kinds
	}

	fn consider_js_doc_tags(&mut self, node_name: &str, tags: &[JsDocTag]) {
		let mut component = None;
		let mut is_feature_action_key = false;
		let mut is_feature_event_key = false;

		for tag in tags {
			if let JsDocTag::Unsupported { value } = tag {
				let mut words = value.split_whitespace().rev().collect::<Vec<_>>();
				let label = words.pop().unwrap();
				let context = words.pop();

				if label == "@component" {
					component = Some(context.map(|inner| inner.to_string()).unwrap_or(format!("{node_name}Render")));
				} else if value == "@feature_event_key" {
					is_feature_event_key = true;
				} else if value == "@feature_action_key" {
					is_feature_action_key = true;
				} else if value == "@feature_component_index" {
					self.component_index_name = Some(node_name.to_string());
				}
			}
		}

		if is_feature_event_key {
			self.event_key_type_name = Some(node_name.to_string());
		}

		if is_feature_action_key {
			self.action_key_type_name = Some(node_name.to_string());
		}

		if let Some(render_name) = component {
			self.components.insert(node_name.to_string(), ComponentInfo { render_name });
		}
	}
}
