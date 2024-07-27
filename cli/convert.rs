use anyhow::{Context, Result};
use deno_doc::{interface::InterfaceDef, ts_type::TsTypeDef, Location};
use log::debug;

use crate::{collect::ComponentInfo, diagnostic::Diagnostic};

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
	KeyedEnum { variants: Vec<EnumProperty> },
	Object { properties: Vec<ObjectProperty> },
}

#[derive(Debug)]
pub struct EnumProperty {
	pub comment: Option<String>,
	pub name: String,
	pub kind: Kind,
}

#[derive(Debug)]
pub struct ObjectProperty {
	pub comment: Option<String>,
	pub name: String,
	pub kind: Kind,
	pub is_optional: bool,
}

#[derive(Debug)]
pub struct Conversion {
	pub kind: Kind,
	pub dependencies: Vec<String>,
}

pub struct ConvertInterfaceParams<'a> {
	pub interface: &'a InterfaceDef,
	pub location: &'a Location,
	pub component: Option<&'a mut ComponentInfo>,
	pub action_key_type_name: Option<&'a str>,
	pub event_key_type_name: Option<&'a str>,
}

pub fn convert_interface(params: ConvertInterfaceParams<'_>) -> Result<Conversion> {
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
		return Diagnostic::start("Interface extensions are not supported, instead specify all properties in the interface body")
			.shift()
			.location(location)
			.build()
			.err();
	}

	if !interface.methods.is_empty() {
		return Diagnostic::start(
			"Methods are not supported in exported interfaces. If this is releated to private client-only functionality, consider \
			inlining the methods in the render function",
		)
		.shift()
		.location(location)
		.build()
		.err();
	}

	for property_def in &interface.properties {
		let type_def = match &property_def.ts_type {
			Some(def) => def,
			None => {
				return Diagnostic::start("Interface property does not have an associated type")
					.shift()
					.location(&property_def.location)
					.build()
					.err();
			}
		};

		let mut conversion = convert_ts_type(ConvertTsTypeParams {
			ts_type: type_def,
			location: &property_def.location,
			component: component.as_deref_mut(),
			action_key_type_name,
			event_key_type_name,
		})
		.with_context(|| {
			Diagnostic::start("Failed to convert interface property ")
				.inline_code(&property_def.name)
				.shift()
				.location(&property_def.location)
				.build()
		})?;

		interface_dependencies.append(&mut conversion.dependencies);

		properties.push(ObjectProperty {
			comment: property_def.js_doc.doc.clone(),
			name: property_def.name.to_string(),
			kind: conversion.kind,
			is_optional: property_def.optional,
		})
	}

	Ok(Conversion {
		kind: Kind::Object { properties },
		dependencies: interface_dependencies,
	})
}

pub struct ConvertTsTypeParams<'a> {
	pub ts_type: &'a TsTypeDef,
	pub location: &'a Location,
	pub component: Option<&'a mut ComponentInfo>,
	pub action_key_type_name: Option<&'a str>,
	pub event_key_type_name: Option<&'a str>,
}

pub fn convert_ts_type(params: ConvertTsTypeParams<'_>) -> Result<Conversion> {
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
			return Diagnostic::start("Use 'unknown' instead of 'any'").shift().location(location).build().err();
		}

		return Diagnostic::start("Unknown keyword '{keyword}'").shift().location(location).build().err();
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
					return Diagnostic::start("Because it is an action key, expected to find 1 type parameter for ")
						.inline_code(action_key_type_name)
						.text(", but found ")
						.text(type_params.len().to_string().as_str())
						.shift()
						.location(location)
						.build()
						.err();
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
					return Diagnostic::start("Because it is an event key, expected to find 1 type parameter for ")
						.inline_code(event_key_type_name)
						.text(", but found ")
						.text(type_params.len().to_string().as_str())
						.shift()
						.location(location)
						.build()
						.err();
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
			return Diagnostic::start("Type ")
				.inline_code(&type_ref.type_name)
				.text(" was supplied type parameters, but this is not supported")
				.shift()
				.location(location)
				.build()
				.err();
		}

		return Ok(Conversion {
			kind: Kind::Ref {
				name: type_ref.type_name.clone(),
			},
			dependencies: Vec::from([type_ref.type_name.clone()]),
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
					return Diagnostic::start("Failed to convert variant ")
						.text(variant_number)
						.text(" in union. Only string literals and keyed objects are supported.")
						.shift()
						.location(location)
						.build()
						.err();
				}
			} else if let Some(type_literal) = &ts_type.type_literal {
				let mut comment = None;
				let mut name = None;
				let mut definition_kind = None;

				for property in &type_literal.properties {
					if &property.name == "type" {
						comment = property.js_doc.doc.clone();

						let type_type = property.ts_type.as_ref().ok_or(
							Diagnostic::start("Expected to find a type associated with the ")
								.inline_code("type")
								.text(" field")
								.shift()
								.location(&property.location)
								.build()
								.error(),
						)?;

						if let Some(literal) = &type_type.literal {
							if let Some(string) = &literal.string {
								name = Some(string.clone())
							} else {
								return Diagnostic::start("The type of the ")
									.inline_code("type")
									.text(" property must be a string literal, as this is a keyed object")
									.shift()
									.location(&property.location)
									.build()
									.err();
							}
						} else {
							return Diagnostic::start("The type of the ")
								.inline_code("type")
								.text(" property must be a string literal, as this is a keyed object")
								.shift()
								.location(&property.location)
								.build()
								.err();
						}
					} else if &property.name == "def" {
						let mut def_conversion = convert_ts_type(ConvertTsTypeParams {
							ts_type: property.ts_type.as_ref().ok_or(
								Diagnostic::start("Expected to find a type assiciated with the ")
									.inline_code("def")
									.text(" field")
									.shift()
									.location(&property.location)
									.build()
									.error(),
							)?,
							location: &property.location,
							component: component.as_deref_mut(),
							action_key_type_name,
							event_key_type_name,
						})
						.with_context(|| {
							Diagnostic::start("Failed to convert property ")
								.inline_code(&property.name)
								.shift()
								.location(&property.location)
								.build()
						})?;

						debug!("{:?}", &def_conversion.dependencies);
						combined_dependencies.append(&mut def_conversion.dependencies);
						definition_kind = Some(def_conversion.kind);
					}
				}

				let name = name.ok_or(
					Diagnostic::start("Union variant ")
						.text(variant_number)
						.text("is not a valid keyed object. No ")
						.inline_code("type")
						.text(" field was found.")
						.shift()
						.location(location)
						.build()
						.error(),
				)?;
				let kind = definition_kind.ok_or(
					Diagnostic::start("Union variant ")
						.text(variant_number)
						.text(" is not a valid keyed object. No ")
						.inline_code("def")
						.text(" field was found")
						.shift()
						.location(location)
						.build()
						.error(),
				)?;

				keyed_variants.push(EnumProperty { comment, name, kind });
			} else {
				return Diagnostic::start("Unsupported enum type in variant ")
					.text(variant_number)
					.text(". Only string literals and keyed objects are supported.")
					.shift()
					.location(location)
					.build()
					.err();
			}

			variant_number += 1;
		}

		if !string_variants.is_empty() && !keyed_variants.is_empty() {
			return Diagnostic::start(
				"Found a union with both string and keyed object variants. This is not allowed.\
				The entire union must be made up of either string literals or keyed objects",
			)
			.shift()
			.location(location)
			.build()
			.err();
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

	if let Some(_) = &ts_type.type_literal {
		return Diagnostic::start("Object literals are not supported for types. Use an interface instead.")
			.shift()
			.location(location)
			.build()
			.err();
	}

	debug!("Encountered an unknown type: {:#?}", ts_type);
	Diagnostic::start("Unsupported type").shift().location(location).build().err()
}
