use anyhow::{Context, Result};
use axum::{http::HeaderMap, response::Html, routing::get, serve, Router};
use log::info;
use tokio::net::TcpListener;

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

	let app = Router::new().route("/", get(move || async { Html(STATIC_HTML) })).route(
		"/bundle.js",
		get(|| async {
			let mut headers = HeaderMap::new();
			headers.insert("content-type", "appliaction/json".parse().unwrap());

			(headers, client_bundle)
		}),
	);

	let listener = TcpListener::bind(("localhost", params.web_port))
		.await
		.with_context(|| format!("failed to bind to localhost:{}", params.web_port))?;

	info!("Serving the static website at http://localhost:{}", params.web_port);

	serve(listener, app).await.context("failed to serve the generated web static platform code")?;

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
