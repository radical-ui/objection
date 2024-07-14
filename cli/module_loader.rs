use anyhow::{anyhow, Context, Error, Result};
use deno_graph::source::{MemoryLoader, Source};
use rand::random;
use serde_json::{from_slice, Value};
use std::{
	env::{self, temp_dir},
	fmt::Write,
	path::Path,
	process::Stdio,
};
use tokio::{
	fs::{read_to_string, write},
	process::Command,
};
use url::Url;

pub async fn load_modules(entry_url: &Url) -> Result<MemoryLoader> {
	// let index_file_path = temp_dir().join(random::<u64>().to_string());
	// let mut index_file_contents = String::new();
	let mut memory_loader = MemoryLoader::default();

	// write(&index_file_path, index_file_contents)
	// 	.await
	// 	.with_context(|| format!("failed to write to {index_file_path:?}"))?;

	let json_graph = get_json_graph(entry_url.as_str()).await?;
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

		if let Some(error_value) = module_object.get("error") {
			let error = error_value.as_str().ok_or(json_read_error("expected `error` key to be a string"))?;

			Err(anyhow!(error.to_string()).context(format!("Failed to load {specifier}")))?
		}

		let local = module_object
			.get("local")
			.ok_or(json_read_error("expected to find a `local` key in module object"))?
			.as_str()
			.ok_or(json_read_error("expected `local` key to be a string"))?;

		let content = read_to_string(local)
			.await
			.with_context(|| format!("tried to read '{local}', the local path for {specifier}"))?;

		memory_loader.add_source(
			specifier,
			Source::Module {
				specifier: specifier.to_string(),
				maybe_headers: None,
				content,
			},
		);
	}

	Ok(memory_loader)
}

fn json_read_error(reason: &str) -> Error {
	anyhow!(format!("Unexpected output from `deno info`: {reason}"))
}

async fn get_json_graph(entry: &str) -> Result<Value> {
	let mut command = Command::new("deno");

	command
		.arg("info")
		.arg("--json")
		.arg(entry)
		.env_clear()
		.env("PATH", env::var("PATH").unwrap())
		.stdout(Stdio::piped())
		.stderr(Stdio::inherit())
		.stdin(Stdio::null());

	let process = command.spawn()?;
	let output = process.wait_with_output().await?;

	Ok(from_slice(&output.stdout)?)
}
