use std::fmt::{self, Display};

use anyhow::Result;
use clap::ValueEnum;
use log::info;

use crate::{collect::Collection, gen_rust::RustGen};

#[derive(Default, Debug, ValueEnum, Clone, Copy)]
pub enum Engine {
	#[default]
	Rust,
}

impl Display for Engine {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_possible_value().unwrap().get_name())
	}
}

impl Engine {
	pub fn get_bindings(&self, collection: &Collection) -> Result<String> {
		match self {
			Self::Rust => {
				let mut gen = RustGen::new(collection)?;
				gen.gen()?;
				info!("Generated rust engine bindings");

				let output = gen.get_output();
				info!("Formatted rust engine bindings");

				Ok(output)
			}
		}
	}
}
