use inflector::Inflector;
use std::{collections::HashMap, fmt::Display};

use crate::{
	collect::Collection,
	convert::Kind,
	diagnostic::{Diagnostic, DiagnosticList},
};

const RUST_RESERVED_WORDS: &[&str] = &[
	"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
	"mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await",
	"dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

#[derive(Debug, Clone, Copy)]
enum ReservationTarget {
	Rust,
}

impl Display for ReservationTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "rust")
	}
}

#[derive(Debug, Clone, Copy)]
enum NameContext {
	Type,
	Property,
	Variant,
}

impl Display for NameContext {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			NameContext::Type => write!(f, "type name"),
			NameContext::Property => write!(f, "property name"),
			NameContext::Variant => write!(f, "enum variant"),
		}
	}
}

pub struct Inspector<'a> {
	collection: &'a Collection,
	reserved_words: HashMap<&'static str, ReservationTarget>,
}

// TODO we should be passing a context along into these functions, so that errors like "type is a reserved word" show where the offensive property is located
impl Inspector<'_> {
	pub fn new<'a>(collection: &'a Collection) -> Inspector<'a> {
		let mut reserved_words = HashMap::new();

		reserved_words.extend(RUST_RESERVED_WORDS.iter().copied().map(|word| (word, ReservationTarget::Rust)));

		Inspector { collection, reserved_words }
	}

	pub fn inspect(self, diagnostic_list: &mut DiagnosticList) {
		for def in self.collection.get_kinds() {
			self.inspect_name(def.name, NameContext::Type, diagnostic_list);
			self.inspect_kind(def.kind, diagnostic_list);
		}
	}

	fn inspect_kind(&self, kind: &Kind, diagnostic_list: &mut DiagnosticList) {
		match kind {
			Kind::Dynamic | Kind::String | Kind::Number | Kind::Bool | Kind::Null | Kind::Ref { .. } => (),
			Kind::ActionKey { data_type } | Kind::EventKey { data_type } => self.inspect_kind(data_type, diagnostic_list),
			Kind::List { of } => self.inspect_kind(of, diagnostic_list),
			Kind::Tuple { items } => {
				for item in items {
					self.inspect_kind(item, diagnostic_list);
				}
			}
			Kind::StringEnum { variants } => {
				for name in variants {
					self.inspect_name(name.as_str(), NameContext::Variant, diagnostic_list)
				}
			}
			Kind::KeyedEnum { variants } => {
				for variant in variants {
					self.inspect_name(&variant.name, NameContext::Variant, diagnostic_list);
					self.inspect_kind(&variant.kind, diagnostic_list);
				}
			}
			Kind::Object { properties } => {
				for property in properties {
					self.inspect_name(&property.name, NameContext::Property, diagnostic_list);
					self.inspect_kind(&property.kind, diagnostic_list);
				}
			}
		}
	}

	fn inspect_name(&self, name: &str, context: NameContext, diagnostic_list: &mut DiagnosticList) {
		let (is_valid_case, expected, expected_type) = match context {
			NameContext::Variant | NameContext::Type => {
				let expected = name.to_pascal_case();

				(name == &expected, expected, "pascal case")
			}
			NameContext::Property => {
				let expected = name.to_camel_case();

				(name == &expected, expected, "camel case")
			}
		};

		if !is_valid_case {
			diagnostic_list.add(
				Diagnostic::start("Invalid case for ")
					.text(context)
					.text(" ")
					.inline_code(name)
					.shift()
					.text("Expected the ")
					.text(expected_type)
					.text(" form of the word: ")
					.inline_code(expected)
					.build(),
			)
		}

		if let Some(reservation_target) = self.reserved_words.get(&name) {
			diagnostic_list.add(
				Diagnostic::start("Use of reserved ")
					.text(context)
					.text(" ")
					.inline_code(name)
					.shift()
					.text("This word is a reserved in ")
					.text(reservation_target)
					.text(", which is an engine that could be targeted")
					.build(),
			)
		}
	}
}
