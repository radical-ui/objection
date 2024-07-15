use anyhow::{anyhow, Context, Error, Result};
use deno_graph::source::{MemoryLoader, Source};
use serde_json::{from_slice, Value};
use std::{env, path::PathBuf, process::Stdio, str::FromStr};
use tokio::{fs::read_to_string, process::Command};
use url::Url;

use crate::bundle::Bundler;

pub async fn load_modules(entry_url: &Url, memory_loader: &mut MemoryLoader, bundler: &mut Bundler) -> Result<()> {
	cache_graph(entry_url).await?;

	let json_graph = get_json_graph(entry_url).await?;
	let json_modules = json_graph
		.as_object()
		.ok_or(json_read_error("json output was not an object"))?
		.get("modules")
		.ok_or(json_read_error("no `modules` key"))?
		.as_array()
		.ok_or(json_read_error("`modules` is not an array"))?;

	for module_value in json_modules {
		let module_object = module_value.as_object().ok_or(anyhow!("Unexpected output from `deno info`"))?;
		let specifier = module_object
			.get("specifier")
			.ok_or(json_read_error("expected to find a `specifier` key in module object"))?
			.as_str()
			.ok_or(json_read_error("expected `specifier` key in module object to be a string"))?;
		let specifier_url = specifier
			.parse::<Url>()
			.with_context(|| json_read_error(&format!("expected specifier '{}' to be a valid url", specifier)))?;

		if let Some(error_value) = module_object.get("error") {
			let error = error_value.as_str().ok_or(json_read_error("expected `error` key to be a string"))?;

			Err(anyhow!(error.to_string()).context(format!("Failed to load {specifier}")))?
		}

		if let Some(dependencies_value) = module_object.get("dependencies") {
			let dependencies = dependencies_value
				.as_array()
				.ok_or(anyhow!(json_read_error("expected `dependencies` to be an array")))?;

			for dependency_value in dependencies {
				let dependency_object = dependency_value
					.as_object()
					.ok_or(json_read_error("expected item in `dependencies` to be an object"))?;

				let dependency_name = dependency_object
					.get("specifier")
					.ok_or(json_read_error("expected to find a `specifier` property in `dependencies` item"))?
					.as_str()
					.ok_or(json_read_error("expected `specifier` in `dependencies` item to be a string"))?;

				let resolved_specifier = dependency_object
					.get("code")
					.ok_or(json_read_error("expected to find a `code` property in `dependencies` item"))?
					.as_object()
					.ok_or(json_read_error("expected `code` property in `dependencies` item to be an object"))?
					.get("specifier")
					.ok_or(json_read_error("expected to find a `code.specifer` property in `dependencies` item"))?
					.as_str()
					.ok_or(json_read_error("expected `code.specifier` property in `dependencies` item to be a string"))?;

				let resolved_specifier_url = resolved_specifier
					.parse::<Url>()
					.with_context(|| json_read_error(&format!("expected `dependencies.*.code.specifier`, '{}' to be a valid url", resolved_specifier)))?;

				bundler.register_dependency(&specifier_url, dependency_name, resolved_specifier_url);
			}
		}

		let local = module_object
			.get("local")
			.ok_or(json_read_error("expected to find a `local` key in module object"))?
			.as_str()
			.ok_or(json_read_error("expected `local` key to be a string"))?
			.parse::<PathBuf>()?;

		let emit = module_object
			.get("emit")
			.map(|value| value.as_str())
			.flatten()
			.map(|item| PathBuf::from_str(item))
			.transpose()?;

		bundler.register_source_file(specifier_url.clone(), emit.unwrap_or(local.clone()));

		let content = read_to_string(&local)
			.await
			.with_context(|| format!("tried to read '{}', the local path for {specifier}", local.to_string_lossy()))?;

		memory_loader.add_source(
			specifier,
			Source::Module {
				specifier: specifier.to_string(),
				maybe_headers: None,
				content,
			},
		);
	}

	Ok(())
}

fn json_read_error(reason: &str) -> Error {
	anyhow!(format!("Unexpected output from `deno info`: {reason}"))
}

async fn get_json_graph(entry_url: &Url) -> Result<Value> {
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

	Ok(from_slice(&output.stdout)?)
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
