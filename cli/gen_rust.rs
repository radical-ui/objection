use inflector::Inflector;
use log::{debug, error};
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{collections::HashSet, iter};
use syn::parse2;

use crate::{
	collect::Collection,
	convert::{EnumProperty, Kind, ObjectProperty},
	diagnostic::Diagnostic,
};

struct GetConstructorInfoParams<'a> {
	struct_name: &'a str,
	argument_prefix: Option<&'a str>,
	properties: &'a [ObjectProperty],
	limit: usize,
}

struct ConstructorInfo {
	construction_body_tokens: TokenStream,
	argument_tokens: TokenStream,
	comment: String,
}

pub struct RustGen<'a> {
	collection: &'a Collection,
	names_generated: HashSet<String>,
	tokens: TokenStream,
}

impl RustGen<'_> {
	pub fn new<'a>(collection: &'a Collection) -> RustGen<'a> {
		RustGen {
			collection,
			names_generated: HashSet::new(),
			tokens: TokenStream::new(),
		}
	}

	pub fn gen(&mut self) {
		self.gen_index();

		for def in self.collection.get_kinds() {
			debug!("Generating {}", def.name);
			let comment = def.comment.unwrap_or("");

			match def.kind {
				Kind::Dynamic
				| Kind::String
				| Kind::Number
				| Kind::Bool
				| Kind::Null
				| Kind::ActionKey { .. }
				| Kind::EventKey { .. }
				| Kind::Ref { .. }
				| Kind::List { .. }
				| Kind::Tuple { .. } => {
					let name = format_ident!("{}", &def.name);

					if !self.has_item(&def.name) {
						let anon_item = self.gen_kind(def.name, None, def.kind);
						let item = quote! {
							#[doc = #comment]
							pub type #name = #anon_item;
						};

						self.add_item(&def.name, item);
					}
				}
				Kind::StringEnum { .. } | Kind::KeyedEnum { .. } | Kind::Object { .. } => {
					self.gen_kind(def.name, def.comment, def.kind);
				}
			};
		}
	}

	pub fn get_output(self) -> String {
		let text = self.tokens.to_string();
		let file = match parse2(self.tokens) {
			Ok(file) => file,
			Err(_) => {
				Diagnostic::start("Invalid rust code was generated. This is a bug.")
					.shift()
					.text("Continuing on with invalid code so that it can be debugged")
					.build()
					.print_error();

				return text;
			}
		};

		unparse(&file)
	}

	fn gen_index(&mut self) {
		let mut index_ident = None;
		let mut inner_tokens = TokenStream::new();

		for (name, info) in self.collection.get_component_info() {
			let name_ident = format_ident!("{name}");

			if info.is_index {
				index_ident = Some(name_ident);
				continue;
			}

			inner_tokens.extend(iter::once(quote! { #name_ident(#name_ident), }));
		}

		if index_ident.is_none() {
			error!("No component index was found during rust code gen. This indicates a failure in the checking step");
		}

		self.tokens.extend(iter::once(quote! {
			pub enum #index_ident {
				#inner_tokens
			}
		}));
	}

	fn gen_kind(&mut self, context_name: &str, comment: Option<&str>, kind: &Kind) -> TokenStream {
		match kind {
			Kind::Dynamic => quote! { serde_json::Value },
			Kind::String => quote! { String },
			Kind::Number => quote! { f64 },
			Kind::Bool => quote! { bool },
			Kind::Null => quote! { () },
			Kind::ActionKey { data_type } => {
				let inner = self.gen_kind(&format!("{context_name}ActionData"), None, &data_type);

				quote! { objection::ActionKey<#inner> }
			}
			Kind::EventKey { data_type } => {
				let inner = self.gen_kind(&format!("{context_name}EventData"), None, &data_type);

				quote! { objection::EventKey<#inner> }
			}
			Kind::Ref { name } => {
				let inner = format_ident!("{}", name);

				quote! { #inner }
			}
			Kind::List { of } => {
				let inner = self.gen_kind(&format!("{context_name}Item"), None, &of);

				quote! { Vec<#inner> }
			}
			Kind::Tuple { items } => {
				let inner = items
					.iter()
					.enumerate()
					.map(|(index, kind)| self.gen_kind(&format!("{context_name}Item{index}"), None, kind));

				quote! { ( #( #inner ),* ) }
			}
			Kind::StringEnum { variants } => {
				let name_ident = format_ident!("{context_name}");

				if !self.has_item(context_name) {
					let variant_idents = variants.iter().map(|item| format_ident!("{item}"));
					let comment_str = comment.unwrap_or_default();
					let item = quote! {
						#[doc = #comment_str]
						#[derive(Debug, serde::Serialize, serde::Deserialize)]
						pub enum #name_ident {
							#( #variant_idents, )*
						}
					};

					self.add_item(context_name, item);
				}

				quote! { #name_ident }
			}
			Kind::KeyedEnum { variants } => {
				let name_ident = format_ident!("{context_name}");

				if !self.has_item(context_name) {
					self.gen_keyed_enum(context_name, comment, &variants)
				}

				quote! { #name_ident }
			}
			Kind::Object { properties } => {
				let name_ident = format_ident!("{context_name}");

				if !self.has_item(context_name) {
					self.gen_struct(context_name, comment, &properties);
				}

				quote! { #name_ident }
			}
		}
	}

	fn has_item(&self, name: &str) -> bool {
		self.names_generated.contains(name)
	}

	fn add_item(&mut self, name: &str, tokens: TokenStream) {
		self.names_generated.insert(name.to_string());
		self.tokens.extend(iter::once(tokens));
	}

	fn gen_keyed_enum(&mut self, context_name: &str, comment: Option<&str>, variants: &[EnumProperty]) {
		let name_ident = format_ident!("{context_name}");
		let mut variant_def_tokens = Vec::new();

		// TODO add constructors for objects that can be constructed
		// let mut constructors = Vec::new();

		for variant in variants {
			let name_ident = format_ident!("{}", &variant.name);
			let kind_tokens = self.gen_kind(
				&get_keyed_enum_variant_context_name(context_name, &variant.name),
				variant.comment.as_deref(),
				&variant.kind,
			);

			variant_def_tokens.push(quote! { #name_ident(#kind_tokens) });
		}

		let comment_str = comment.unwrap_or_default();
		let item = quote! {
			#[doc = #comment_str]
			#[derive(Debug, serde::Serialize, serde::Deserialize)]
			pub enum #name_ident {
				#( #variant_def_tokens, )*
			}

			impl #name_ident {

			}
		};

		self.add_item(context_name, item);
	}

	fn gen_struct(&mut self, context_name: &str, comment: Option<&str>, properties: &[ObjectProperty]) {
		let name_ident = format_ident!("{context_name}");
		let mut property_def_tokens = TokenStream::new();
		let mut methods = TokenStream::new();

		let constructor_tokens = {
			let info = self.get_constructor_info(GetConstructorInfoParams {
				struct_name: context_name,
				argument_prefix: None,
				properties,
				limit: 3,
			});

			info.map(
				|ConstructorInfo {
				     construction_body_tokens,
				     argument_tokens,
				     comment,
				 }| {
					let full_comment = format!("Construct a new {context_name}.\n\n{comment}");

					quote! {
						#[doc = #full_comment]
						pub fn new(#argument_tokens) -> #name_ident {
							#name_ident { #construction_body_tokens }
						}
					}
				},
			)
		};

		for property in properties {
			let snake_property_name = property.name.to_snake_case();
			let snake_property_ident = format_ident!("{}", &snake_property_name);
			let comment_tokens = property.comment.as_deref().map(|text| quote! { #[doc = #text] });
			let (resolved_kind, _) = self.collection.resolve_kind(&property.kind);

			let bare_kind_tokens = self.gen_kind(
				&get_struct_property_context_name(context_name, &property.name),
				property.comment.as_deref(),
				&property.kind,
			);

			let kind_tokens_type = if property.is_optional {
				let clone = bare_kind_tokens.clone();
				quote! { Option<#clone> }
			} else {
				bare_kind_tokens.clone()
			};

			let default_method = quote! {
				pub fn #snake_property_ident(mut self, #snake_property_ident: #kind_tokens_type) -> #name_ident {
					self.#snake_property_ident = #snake_property_ident;

					self
				}
			};

			methods.extend(iter::once(if let Kind::Bool = resolved_kind {
				let property_if_ident = format_ident!("{snake_property_name}_if");

				quote! {
					pub fn #snake_property_ident(mut self) -> #name_ident {
						self.#snake_property_ident = true;

						self
					}

					pub fn #property_if_ident(mut self, #snake_property_ident: #kind_tokens_type) -> #name_ident {
						self.#snake_property_ident = #snake_property_ident;

						self
					}
				}
			} else if let Kind::Object { properties } = resolved_kind {
				self.get_constructor_info(GetConstructorInfoParams {
					struct_name: context_name,
					argument_prefix: Some(&snake_property_name),
					properties,
					limit: 3,
				})
				.map(
					|ConstructorInfo {
					     construction_body_tokens,
					     argument_tokens,
					     comment,
					 }| {
						let full_name_ident = format_ident!("{snake_property_ident}_full");
						let option_wrapped_construction_tokens = if property.is_optional {
							quote! { Some(#bare_kind_tokens { #construction_body_tokens }) }
						} else {
							quote! { #bare_kind_tokens { #construction_body_tokens } }
						};
						let option_wrapped_snake_property = if property.is_optional {
							quote! {
								Some(#snake_property_ident)
							}
						} else {
							quote! { #snake_property_ident }
						};

						// TODO comments
						quote! {
							pub fn #snake_property_ident(mut self, #argument_tokens) -> #name_ident {
								// TODO these kind tokens here may not work when there is a level of indirection with aliases
								self.#snake_property_ident = #option_wrapped_construction_tokens;

								self
							}

							pub fn #full_name_ident(mut self, #snake_property_ident: #bare_kind_tokens) -> #name_ident {
								self.#snake_property_ident = #option_wrapped_snake_property;

								self
							}
						}
					},
				)
				.unwrap_or(default_method)
			} else {
				default_method
			}));

			let def_tokens = quote! {
				#comment_tokens
				pub #snake_property_ident: #kind_tokens_type,
			};

			property_def_tokens.extend(iter::once(def_tokens));
		}

		let comment_tokens = comment.map(|text| quote! { #[doc = #text] });

		let item = quote! {
			#comment_tokens
			#[derive(Debug, serde::Serialize, serde::Deserialize)]
			#[serde(rename_all = "camelCase")]
			pub struct #name_ident { #property_def_tokens }

			impl #name_ident {
				#constructor_tokens

				#methods
			}
		};

		self.add_item(context_name, item)
	}

	fn get_constructor_info(&mut self, params: GetConstructorInfoParams<'_>) -> Option<ConstructorInfo> {
		let GetConstructorInfoParams {
			struct_name,
			argument_prefix,
			properties,
			limit,
		} = params;

		let mut argument_tokens = TokenStream::new();
		let mut construction_body_tokens = TokenStream::new();
		let mut comment = String::new();
		let mut arguments_so_far = 0_usize;

		for property in properties {
			let property_name_ident = format_ident!("{}", &property.name.to_snake_case());

			if property.is_optional {
				construction_body_tokens.extend(iter::once(quote! { #property_name_ident: None, }));

				continue;
			}

			if arguments_so_far == limit {
				return None;
			}

			let argument_name_ident = match argument_prefix {
				Some(prefix) => format_ident!("{prefix}_{}", &property.name.to_snake_case()),
				None => format_ident!("{}", &property.name.to_snake_case()),
			};

			let type_ = self.gen_kind(
				&get_struct_property_context_name(struct_name, &property.name),
				property.comment.as_deref(),
				&property.kind,
			);

			construction_body_tokens.extend(iter::once(quote! { #property_name_ident: #argument_name_ident, }));
			argument_tokens.extend(iter::once(quote! { #argument_name_ident: #type_, }));

			if let Some(property_comment) = &property.comment {
				comment.push_str(&format!("Argument `{}`: {property_comment}\n\n", &property.name));
			}

			arguments_so_far += 1;
		}

		Some(ConstructorInfo {
			construction_body_tokens,
			argument_tokens,
			comment,
		})
	}
}

fn get_struct_property_context_name(struct_context_name: &str, property_name: &str) -> String {
	// all property names are camel case, but all property names must be pascal case
	format!("{struct_context_name}{}", property_name.to_pascal_case())
}

fn get_keyed_enum_variant_context_name(enum_context_name: &str, variant_name: &str) -> String {
	// all variant names must be pascal case, so nothing to do here
	format!("{enum_context_name}{variant_name}")
}
