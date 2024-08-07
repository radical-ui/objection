use anyhow::Result;
use log::warn;

const STATIC_HTML: &str = include_str!("web_static_index.html");

use crate::{
	build::{build, Build, BuildOptions},
	writer::{FileWriter, Writer},
};

#[derive(Debug, Clone, Copy)]
pub struct RunWebStaticParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub web_port: u16,
	pub reload: bool,
	pub bindings_writer: &'a FileWriter,
}

pub async fn run_web_static(params: RunWebStaticParams<'_>) -> Result<()> {
	let Build { client_bundle, bindings } = build(params.build_options).await?;

	params.bindings_writer.write(bindings).await?;

	warn!("should be serving the bundle on port {}", params.web_port);

	Ok(())
}

#[derive(Debug, Clone, Copy)]
pub struct BuildWebStaticParams<'a> {
	pub build_options: BuildOptions<'a>,
	pub bindings_writer: &'a FileWriter,
	pub output_writer: &'a Writer,
}

pub async fn build_web_static(params: BuildWebStaticParams<'_>) -> Result<()> {
	let Build { client_bundle, bindings } = build(params.build_options).await?;

	params.bindings_writer.write(bindings).await?;
	params.output_writer.write_file("index.html", STATIC_HTML).await?;
	params.output_writer.write_file("bundle.js", client_bundle).await?;

	Ok(())
}
