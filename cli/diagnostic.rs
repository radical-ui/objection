use anstyle::Style;
use anyhow::{Error, Result};
use colored::Colorize;
use deno_doc::Location;
use log::{error, warn};
use std::fmt::{Display, Write};

pub struct DiagnosticList {
	diagnostics: Vec<Diagnostic>,
}

impl DiagnosticList {
	pub fn new() -> DiagnosticList {
		DiagnosticList { diagnostics: Vec::new() }
	}

	pub fn add(&mut self, diagnostic: Diagnostic) {
		self.diagnostics.push(diagnostic)
	}

	pub fn add_error(&mut self, error: Error) {
		self.diagnostics.push(Diagnostic::from_error(error))
	}

	pub fn flush(&mut self, operation: impl Display) -> Result<()> {
		let error_count = self.diagnostics.len();

		for diagnostic in self.diagnostics.drain(..) {
			diagnostic.print_error();
		}

		if error_count > 0 {
			Diagnostic::start("Could not ")
				.text(operation)
				.text(" due to ")
				.text(error_count)
				.text(" previous error")
				.text(if error_count == 1 { "" } else { "s" })
				.build()
				.err()
		} else {
			Ok(())
		}
	}
}

pub struct Diagnostic(String);

impl Diagnostic {
	pub fn from_error(error: Error) -> Diagnostic {
		let mut string = String::new();
		let _ = write!(&mut string, "{:?}", error);

		let newline_index = string.find('\n');
		if let Some(newline_index) = newline_index {
			let mut new_string = String::new();
			let _ = write!(new_string, "{FORE_STYLE}{}{FORE_STYLE:#}{}", &string[..newline_index], &string[newline_index..]);

			string = new_string;
		}

		Diagnostic(string)
	}

	pub fn start(initial_message: impl Display) -> DiagnosticBuilder {
		DiagnosticBuilder::new(Diagnostic(String::new())).text(initial_message)
	}

	pub fn error(self) -> Error {
		Error::msg(self.0)
	}

	pub fn err<T>(self) -> Result<T> {
		Err(self.error())
	}

	pub fn print_error(self) {
		error!("{}", self.error());
	}

	pub fn print_warn(self) {
		warn!("{}", self.0)
	}
}

impl Display for Diagnostic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

const FORE_STYLE: Style = Style::new().bold();

pub struct DiagnosticBuilder {
	diagnostic: Diagnostic,
	did_shift: bool,
}

impl DiagnosticBuilder {
	pub fn new(mut diagnostic: Diagnostic) -> DiagnosticBuilder {
		write!(&mut diagnostic.0, "{FORE_STYLE}").unwrap();

		DiagnosticBuilder { diagnostic, did_shift: false }
	}

	pub fn inline_code(mut self, code: impl Display) -> DiagnosticBuilder {
		write!(&mut self.diagnostic.0, "`{code}`").unwrap();

		self
	}

	pub fn shift(mut self) -> DiagnosticBuilder {
		write!(&mut self.diagnostic.0, "{FORE_STYLE:#}\n  {}", "--> ".bold().blue()).unwrap();
		self.did_shift = true;

		self
	}

	pub fn text(mut self, text: impl Display) -> DiagnosticBuilder {
		write!(&mut self.diagnostic.0, "{text}").unwrap();

		self
	}

	pub fn location(mut self, location: &Location) -> DiagnosticBuilder {
		write!(&mut self.diagnostic.0, "{}:{}:{}", &location.filename, &location.line, &location.col).unwrap();

		self
	}

	pub fn build(self) -> Diagnostic {
		self.diagnostic
	}

	pub fn join_map<T>(mut self, iter: impl Iterator<Item = T>, func: impl Fn(Self, T) -> Self) -> Self {
		let mut first = true;

		for item in iter {
			if first {
				first = false;
			} else {
				write!(&mut self.diagnostic.0, ", ").unwrap();
			}

			self = func(self, item)
		}

		self
	}
}
