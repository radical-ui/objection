use anyhow::{anyhow, Error, Result};
use colored::Colorize;
use deno_doc::Location;
use log::warn;

pub fn contextual_format(main: &str, context: &str) -> String {
	format!("{}\n  {} {}\n", main.bold(), "-->".bold().blue(), context)
}

pub fn local_warn(message: &str, location: &Location) {
	warn!("{:?}", local_error(message, location))
}

pub fn local_error(message: &str, location: &Location) -> Error {
	anyhow!("{}", local_message(message, location))
}

pub fn local_err<T>(message: &str, location: &Location) -> Result<T> {
	Err(local_error(message, location))
}

pub fn local_message(message: &str, location: &Location) -> String {
	let file = format!("{}:{}:{}", location.filename, location.line, location.col);

	contextual_format(message, &file)
}
