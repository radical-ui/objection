use anyhow::{anyhow, Context, Result};
use deno_graph::source::{MemoryLoader, Source};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::{env, path::PathBuf, process::Stdio};
use tokio::{fs::read_to_string, process::Command};
use url::Url;

use crate::bundle::Bundler;

pub async fn load_modules(entry_url: &Url, memory_loader: &mut MemoryLoader, bundler: &mut Bundler) -> Result<()> {
	cache_graph(entry_url).await?;

	let info_graph = InfoGraph::load(entry_url).await?;

	for module in info_graph.modules {
		if let Some(error) = module.error {
			return Err(anyhow!(error).context(format!("Failed to load {}", module.specifier)));
		}

		for dependency in module.dependencies {
			bundler.register_dependency(&module.specifier, dependency.specifier, dependency.resolution.specifier);
		}

		let local = module.local.ok_or(anyhow!("Expected there to be a local path because there was no error"))?;

		bundler.register_source_file(module.specifier.clone(), module.emit.as_ref().unwrap_or(&local));

		let content = read_to_string(&local)
			.await
			.with_context(|| format!("tried to read '{}', the local path for {}", local.to_string_lossy(), module.specifier))?;

		let specifier_string = module.specifier.to_string();

		memory_loader.add_source(
			module.specifier,
			Source::Module {
				specifier: specifier_string,
				maybe_headers: None,
				content,
			},
		);
	}

	Ok(())
}

/// A resolved module dependency
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoGraphModuleDependencyResolution {
	/// The url of the
	pub specifier: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoGraphModuleDependency {
	/// The raw specifier that the user used to refer to this particular dependency
	pub specifier: String,

	/// The resolved dependency
	#[serde(rename = "code")]
	pub resolution: InfoGraphModuleDependencyResolution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoGraphModule {
	/// The full specifier of this module
	pub specifier: Url,

	/// The local path to this module's source code. It will be a path to the cache folder if `specifier` is not a local url. Would only be `None`
	/// if there is an `error`
	#[serde(default)]
	pub local: Option<PathBuf>,

	/// The error that occurred while evaluating this module
	#[serde(default)]
	pub error: Option<String>,

	/// The local path to this module's transpiled code. May be unspecified if the source does not need to be transpiled (it's not typescript or jsx),
	/// or if the module has not been cached
	#[serde(default)]
	pub emit: Option<PathBuf>,

	/// The modules that this module imports
	#[serde(default)]
	pub dependencies: Vec<InfoGraphModuleDependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoGraph {
	pub roots: Vec<Url>,
	pub modules: Vec<InfoGraphModule>,
}

impl InfoGraph {
	pub async fn load(entry_url: &Url) -> Result<InfoGraph> {
		let mut command = Command::new("deno");

		command
			.arg("info")
			.arg("--json")
			.arg(entry_url.as_str())
			.env_clear()
			.env("PATH", env::var("PATH").unwrap())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.stdin(Stdio::null());

		let process = command.spawn()?;
		let output = process.wait_with_output().await?;

		if !output.status.success() {
			return Err(anyhow!(
				"failed to get info on module graph due to `deno info` exiting with a non-zero exit code"
			));
		}

		Ok(from_slice(&output.stdout)
			.context("failed to deserialize the json output of `deno info`; this is probably caused by a regression in deno itself")?)
	}
}

async fn cache_graph(entry_url: &Url) -> Result<()> {
	let mut command = Command::new("deno");

	command
		.arg("cache")
		.arg(entry_url.as_str())
		.env_clear()
		.env("PATH", env::var("PATH").unwrap())
		.stdout(Stdio::inherit())
		.stdout(Stdio::inherit())
		.stdin(Stdio::null());

	let mut process = command.spawn()?;
	let status = process.wait().await?;

	if !status.success() {
		return Err(anyhow!("failed to cache module graph due to `deno cache` exiting with a non-zero exit code"));
	}

	Ok(())
}
