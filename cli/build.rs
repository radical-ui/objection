use anyhow::{bail, Context, Result};
use deno_graph::source::MemoryLoader;
use log::{error, info};
use url::Url;

use crate::{
	asset_loader::AssetsLoader,
	bundle::{BundleParams, Bundler},
	collect::Collection,
	diagnostic::DiagnosticList,
	engine::Engine,
	inspect::Inspector,
	module_loader::load_modules,
};

#[derive(Debug, Clone, Copy)]
pub struct BuildOptions<'a> {
	pub runtime: &'a Url,
	pub bundler: &'a Url,
	pub engine_url: &'a Url,
	pub engine: Engine,
}

pub struct Build {
	pub client_bundle: String,
	pub bindings: String,
	pub assets_loader: AssetsLoader,
}

pub async fn build(diagnostic_list: &mut DiagnosticList, options: BuildOptions<'_>) -> Result<Build> {
	let mut memory_loader = MemoryLoader::default();
	let mut bundler = Bundler::default();
	let mut collection = Collection::default();

	load_modules(options.runtime, &mut memory_loader, &mut bundler).await?;
	info!("Loaded runtime");

	collection.collect(&options.runtime, &memory_loader).await?;
	collection.check_components();

	let errors = collection.get_errors();
	let error_count = errors.len();

	for error in errors {
		error!("{:?}", error);
	}

	if error_count > 0 {
		bail!(
			"could not mount runtime due to {} previous error{}",
			error_count,
			if error_count == 1 { "" } else { "s" }
		);
	}

	info!("Mounted runtime");

	let inspector = Inspector::new(&collection);
	inspector.inspect(diagnostic_list);

	diagnostic_list.flush("validate runtime")?;
	info!("Validated runtime");

	let client_bundle = bundler
		.bundle(BundleParams {
			bundler_url: options.bundler,
			runtime_url: options.runtime,
			collection: &collection,
		})
		.await?;
	info!("Bundled runtime");

	let bindings = options.engine.get_bindings(&collection)?;

	let mut assets_loader = collection.finish();
	assets_loader.load(diagnostic_list).await.context("Failed to load assets")?;
	diagnostic_list.flush("load assets")?;
	info!("Loaded assets");

	Ok(Build {
		client_bundle,
		bindings,
		assets_loader,
	})
}
