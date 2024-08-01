use anyhow::{anyhow, bail, Result};
use inflector::Inflector;
use log::debug;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
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
	comment: Option<String>,
}

#[derive(Debug, Clone)]
enum KindContext {
	/// This is a type in a struct
	Type,
	/// This is a type in a value
	CallSignature,
	/// This is not a type, but the start of a construction. For example, `StructName` in `StructName { ... properties ... }`.
	/// `existing_value_expression` is
	ConstructorKey,
	/// The value representation of the kind. Generally this is just `existing_value_representation`, but if
	Value { existing_value_expression: TokenStream },
}

pub struct RustGen<'a> {
	collection: &'a Collection,
	names_generated: HashSet<String>,
	index_name: &'a str,
	tokens: TokenStream,
}

impl RustGen<'_> {
	pub fn new<'a>(collection: &'a Collection) -> Result<RustGen<'a>> {
		let index_name = collection
			.get_component_info()
			.iter()
			.find_map(|(name, info)| if info.is_index { Some(*name) } else { None })
			.ok_or(anyhow!(
				"No component index was found during rust code gen. This indicates a failure in the checking step"
			))?;

		Ok(RustGen {
			collection,
			index_name,
			names_generated: HashSet::new(),
			tokens: TokenStream::new(),
		})
	}

	pub fn gen(&mut self) -> Result<()> {
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
						let anon_item = self.gen_kind(def.name, None, def.kind, KindContext::Type)?;
						let item = quote! {
							#[doc = #comment]
							pub type #name = #anon_item;
						};

						self.add_item(&def.name, item);
					}
				}
				Kind::StringEnum { .. } | Kind::KeyedEnum { .. } | Kind::Object { .. } => {
					self.gen_kind(def.name, def.comment, def.kind, KindContext::Type)?;
				}
			};
		}

		Ok(())
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
		let index_ident = format_ident!("{}", self.index_name);
		let mut inner_tokens = TokenStream::new();

		for (name, info) in self.collection.get_component_info() {
			if info.is_index {
				continue;
			}

			let comment = self.collection.get_comment(name).map(|comment| quote! { #[doc = #comment] });
			let name_ident = format_ident!("{name}");

			inner_tokens.extend(iter::once(quote! {
				#comment
				#name_ident(Box<#name_ident>),
			}));

			self.tokens.extend(iter::once(quote! {
				impl objection::IntoComponentIndex for #name_ident {
					type Index = #index_ident;

					fn into(self) -> #index_ident {
						#index_ident::#name_ident(Box::new(self))
					}
				}
			}));
		}

		self.tokens.extend(iter::once(quote! {
			#[derive(Debug, serde::Serialize, serde::Deserialize)]
			#[serde(tag = "type", content = "def")]
			pub enum #index_ident {
				#inner_tokens
			}

			impl objection::ComponentIndex for #index_ident {
				fn to_value(self) -> serde_json::Value {
					serde_json::to_value(self).unwrap()
				}
			}
		}));
	}

	fn gen_kind(&mut self, context_name: &str, comment: Option<&str>, kind: &Kind, context: KindContext) -> Result<TokenStream> {
		Ok(match kind {
			Kind::Dynamic => match context {
				KindContext::Type | KindContext::CallSignature => quote! { serde_json::Value },
				KindContext::ConstructorKey => bail!("A dynamic value cannot be constructed via a key"),
				KindContext::Value { existing_value_expression } => existing_value_expression,
			},
			Kind::String => match context {
				KindContext::Type => quote! { String },
				KindContext::CallSignature => quote! { impl Into<String> },
				KindContext::ConstructorKey => bail!("A string cannot be constructed via a key"),
				KindContext::Value { existing_value_expression } => quote! { #existing_value_expression.into() },
			},
			Kind::Number => match context {
				KindContext::Type | KindContext::CallSignature => quote! { f64 },
				KindContext::ConstructorKey => bail!("A number cannot be constructed via a key"),
				KindContext::Value { existing_value_expression } => existing_value_expression,
			},
			Kind::Bool => match context {
				KindContext::Type | KindContext::CallSignature => quote! { bool },
				KindContext::ConstructorKey => bail!("A boolean cannot be constructed via a key"),
				KindContext::Value { existing_value_expression } => existing_value_expression,
			},
			Kind::Null => quote! { () },
			Kind::ActionKey { data_type } => {
				let inner = self.gen_kind(&format!("{context_name}ActionData"), None, &data_type, KindContext::Type)?;

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { objection::ActionKey<#inner> },
					KindContext::ConstructorKey => bail!("An event key cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
			Kind::EventKey { data_type } => {
				let inner = self.gen_kind(&format!("{context_name}EventData"), None, &data_type, KindContext::Type)?;

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { objection::EventKey<#inner> },
					KindContext::ConstructorKey => bail!("An event key cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
			Kind::Ref { name } => {
				let inner = format_ident!("{}", name);

				match context {
					KindContext::Type | KindContext::ConstructorKey {} => inner.into_token_stream(),
					KindContext::CallSignature => {
						if name == self.index_name {
							let index_ident = format_ident!("{}", self.index_name);

							quote! { impl objection::IntoComponentIndex<Index = #index_ident> }
						} else {
							inner.into_token_stream()
						}
					}
					KindContext::Value { existing_value_expression } => {
						if name == self.index_name {
							quote! { #existing_value_expression.into() }
						} else {
							existing_value_expression
						}
					}
				}
			}
			Kind::List { of } => {
				let inner = self.gen_kind(&format!("{context_name}Item"), None, &of, KindContext::Type)?;

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { Vec<#inner> },
					KindContext::ConstructorKey => bail!("A list cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
			Kind::Tuple { items } => {
				let inner = items
					.iter()
					.enumerate()
					.map(|(index, kind)| self.gen_kind(&format!("{context_name}Item{index}"), None, kind, context.clone()))
					.collect::<Result<Vec<_>>>()?;

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { ( #( #inner ),* ) },
					KindContext::ConstructorKey => bail!("A tuple cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
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

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { #name_ident },
					KindContext::ConstructorKey => bail!("A string enum cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
			Kind::KeyedEnum { variants } => {
				let name_ident = format_ident!("{context_name}");

				if !self.has_item(context_name) {
					self.gen_keyed_enum(context_name, comment, &variants)?
				}

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { quote! { #name_ident } },
					KindContext::ConstructorKey => bail!("A keyed enum cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
			Kind::Object { properties } => {
				let name_ident = format_ident!("{context_name}");

				if !self.has_item(context_name) && self.index_name != context_name {
					self.gen_struct(context_name, comment, &properties)?;
				}

				match context {
					KindContext::Type | KindContext::CallSignature => quote! { #name_ident },
					KindContext::ConstructorKey => bail!("An object cannot be constructed via a key"),
					KindContext::Value { existing_value_expression } => existing_value_expression,
				}
			}
		})
	}

	fn has_item(&self, name: &str) -> bool {
		self.names_generated.contains(name)
	}

	fn add_item(&mut self, name: &str, tokens: TokenStream) {
		self.names_generated.insert(name.to_string());
		self.tokens.extend(iter::once(tokens));
	}

	fn gen_keyed_enum(&mut self, context_name: &str, comment: Option<&str>, variants: &[EnumProperty]) -> Result<()> {
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
				KindContext::Type,
			)?;

			variant_def_tokens.push(quote! { #name_ident(#kind_tokens) });
		}

		let comment_str = comment.unwrap_or_default();
		let item = quote! {
			#[doc = #comment_str]
			#[derive(Debug, serde::Serialize, serde::Deserialize)]
			#[serde(tag = "type", content = "def")]
			pub enum #name_ident {
				#( #variant_def_tokens, )*
			}

			impl #name_ident {

			}
		};

		self.add_item(context_name, item);

		Ok(())
	}

	fn gen_struct(&mut self, context_name: &str, comment: Option<&str>, properties: &[ObjectProperty]) -> Result<()> {
		let name_ident = format_ident!("{context_name}");
		let mut property_def_tokens = TokenStream::new();
		let mut methods = TokenStream::new();

		let constructor_tokens = {
			let info = self.get_constructor_info(GetConstructorInfoParams {
				struct_name: context_name,
				argument_prefix: None,
				properties,
				limit: 3,
			})?;

			info.map(
				|ConstructorInfo {
				     construction_body_tokens,
				     argument_tokens,
				     comment,
				 }| {
					let mut full_comment = format!("Construct a new {context_name}.");

					if let Some(comment) = comment {
						full_comment.push_str(&format!("\n\n{comment}"));
					}

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
			let (resolved_kind, resolved_name) = self.collection.resolve_kind(&property.kind);
			let property_context_name = get_struct_property_context_name(context_name, &property.name);

			let kind_type_tokens = optional_type_if(
				property.is_optional,
				self.gen_kind(&property_context_name, property.comment.as_deref(), &property.kind, KindContext::Type)?,
			);
			let kind_call_signature_tokens = self.gen_kind(&property_context_name, property.comment.as_deref(), &property.kind, KindContext::CallSignature)?;
			let kind_value_tokens = optional_value_if(
				property.is_optional,
				self.gen_kind(
					&property_context_name,
					property.comment.as_deref(),
					&property.kind,
					KindContext::Value {
						existing_value_expression: snake_property_ident.to_token_stream(),
					},
				)?,
			);

			let default_method = quote! {
				pub fn #snake_property_ident(mut self, #snake_property_ident: #kind_call_signature_tokens) -> #name_ident {
					self.#snake_property_ident = #kind_value_tokens;

					self
				}
			};

			methods.extend(iter::once(if resolved_name == Some(self.index_name) {
				default_method
			} else if let Kind::Bool = resolved_kind {
				let property_if_ident = format_ident!("{snake_property_name}_if");

				let kind_value_tokens_default = optional_value_if(
					property.is_optional,
					self.gen_kind(
						&property_context_name,
						property.comment.as_deref(),
						&property.kind,
						KindContext::Value {
							existing_value_expression: quote! { true },
						},
					)?,
				);

				quote! {
					pub fn #snake_property_ident(mut self) -> #name_ident {
						self.#snake_property_ident = #kind_value_tokens_default;

						self
					}

					pub fn #property_if_ident(mut self, #snake_property_ident: #kind_call_signature_tokens) -> #name_ident {
						self.#snake_property_ident = #kind_value_tokens;

						self
					}
				}
			} else if let Kind::Object { properties } = resolved_kind {
				self.get_constructor_info(GetConstructorInfoParams {
					struct_name: context_name,
					argument_prefix: Some(&snake_property_name),
					properties,
					limit: 3,
				})?
				.map(
					|ConstructorInfo {
					     construction_body_tokens,
					     argument_tokens,
					     comment,
					 }|
					 -> Result<_> {
						let kind_constructor_key_tokens =
							self.gen_kind(&property_context_name, property.comment.as_deref(), &property.kind, KindContext::ConstructorKey)?;

						let full_name_ident = format_ident!("{snake_property_ident}_full");
						let wrapped_construction_tokens =
							optional_value_if(property.is_optional, quote! { #kind_constructor_key_tokens { #construction_body_tokens } });
						let comment_tokens = comment.map(|comment| quote! { #[doc = #comment] });

						Ok(quote! {
							#comment_tokens
							pub fn #snake_property_ident(mut self, #argument_tokens) -> #name_ident {
								self.#snake_property_ident = #wrapped_construction_tokens;

								self
							}

							#comment_tokens
							pub fn #full_name_ident(mut self, #snake_property_ident: #kind_call_signature_tokens) -> #name_ident {
								self.#snake_property_ident = #kind_value_tokens;

								self
							}
						})
					},
				)
				.transpose()?
				.unwrap_or(default_method)
			} else {
				default_method
			}));

			let def_tokens = quote! {
				#comment_tokens
				pub #snake_property_ident: #kind_type_tokens,
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

		self.add_item(context_name, item);

		Ok(())
	}

	fn get_constructor_info(&mut self, params: GetConstructorInfoParams<'_>) -> Result<Option<ConstructorInfo>> {
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
			let property_context_name = get_struct_property_context_name(struct_name, &property.name);

			if property.is_optional {
				construction_body_tokens.extend(iter::once(quote! { #property_name_ident: None, }));

				continue;
			}

			if arguments_so_far == limit {
				return Ok(None);
			}

			let argument_name_ident = match argument_prefix {
				Some(prefix) => format_ident!("{prefix}_{}", &property.name.to_snake_case()),
				None => format_ident!("{}", &property.name.to_snake_case()),
			};

			let call_signature_tokens = self.gen_kind(&property_context_name, property.comment.as_deref(), &property.kind, KindContext::CallSignature)?;
			let value_tokens = self.gen_kind(
				&property_context_name,
				property.comment.as_deref(),
				&property.kind,
				KindContext::Value {
					existing_value_expression: argument_name_ident.to_token_stream(),
				},
			)?;

			construction_body_tokens.extend(iter::once(quote! { #property_name_ident: #value_tokens, }));
			argument_tokens.extend(iter::once(quote! { #argument_name_ident: #call_signature_tokens, }));

			if let Some(property_comment) = &property.comment {
				comment.push_str(&format!("Argument `{}`: {property_comment}\n\n", &property.name));
			}

			arguments_so_far += 1;
		}

		Ok(Some(ConstructorInfo {
			construction_body_tokens,
			argument_tokens,
			comment: if comment.is_empty() { None } else { Some(comment) },
		}))
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

fn optional_type_if(condition: bool, inner: TokenStream) -> TokenStream {
	if condition {
		quote! { Option<#inner> }
	} else {
		inner
	}
}

fn optional_value_if(condition: bool, inner: TokenStream) -> TokenStream {
	if condition {
		quote! { Some(#inner) }
	} else {
		inner
	}
}
