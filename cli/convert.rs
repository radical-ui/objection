use anyhow::{Context, Result};
use deno_doc::{interface::InterfaceDef, ts_type::TsTypeDef, Location};
use log::debug;

use crate::{
	collect::ComponentInfo,
	print::{local_err, local_error, local_message},
};

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
pub struct Property {
	pub comment: Option<String>,
	pub name: String,
	pub kind: Kind,
}

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
