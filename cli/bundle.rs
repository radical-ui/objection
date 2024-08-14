use aho_corasick::AhoCorasick;
use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::to_string;
use std::{collections::HashMap, env, fmt::Write, path::PathBuf, process::Stdio};
use tokio::{io::AsyncWriteExt, process::Command};
use url::Url;

use crate::collect::Collection;

const RUNTIME_ENTRY: &str = include_str!("runtime_entry.js");

#[derive(Debug, Serialize, Default)]
struct BundleManifest {
	resolutions: HashMap<Url, HashMap<String, Url>>,
	source_files: HashMap<Url, PathBuf>,
}

pub struct BundleParams<'a> {
	pub bundler_url: &'a Url,
	pub runtime_url: &'a Url,
	pub collection: &'a Collection,
}

#[derive(Debug, Default)]
pub struct Bundler {
	manifest: BundleManifest,
}

impl Bundler {
	pub fn register_dependency(&mut self, host: &Url, dependency: impl Into<String>, specifier: impl Into<Url>) {
		if let Some(dependencies) = self.manifest.resolutions.get_mut(host) {
			dependencies.insert(dependency.into(), specifier.into());
		} else {
			self.manifest
				.resolutions
				.insert(host.to_owned(), HashMap::from([(dependency.into(), specifier.into())]));
		}
	}

	pub fn register_source_file(&mut self, source: impl Into<Url>, file: impl Into<PathBuf>) {
		self.manifest.source_files.insert(source.into(), file.into());
	}

	pub async fn bundle(self, params: BundleParams<'_>) -> Result<String> {
		let imports = {
			let mut js = String::new();

			write!(js, "import {{ createStarter, ")?;

			for (_, info) in params.collection.get_component_info() {
				write!(js, "{}, ", info.render_name)?;
			}

			write!(js, " }} from '{}'", params.runtime_url)?;

			js
		};

		let component_cases = {
			let mut js = String::new();

			for (name, info) in params.collection.get_component_info() {
				write!(
					js,
					"\tif (component.type === '{}') return {{ func: {}, params: component.def }}\n",
					name, &info.render_name
				)?;
			}

			write!(js, "\tthrow new Error('Unknown component type: ' + component.type)\n")?;

			js
		};

		let entry = AhoCorasick::new(&["\"IMPORTS\"", "\"COMPONENT_CASES\""])?.replace_all(RUNTIME_ENTRY, &[imports, component_cases]);

		self.run_bundle_command(params.bundler_url, entry).await
	}

	async fn run_bundle_command(self, bundler_url: &Url, entry_code: impl Into<String>) -> Result<String> {
		let mut command = Command::new("deno");

		command
			.arg("run")
			.arg("--allow-read")
			.arg(bundler_url.as_str())
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.env_clear()
			.env("PATH", env::var("PATH").unwrap());

		let mut process = command.spawn()?;
		let json = to_string(&(entry_code.into(), self.manifest))?;

		let mut stdin = process.stdin.take().unwrap();
		stdin.write_all(json.as_bytes()).await?;
		drop(stdin);

		let output = process.wait_with_output().await?;

		if !output.status.success() {
			bail!("Failed to bundle runtime")
		}

		Ok(String::from_utf8(output.stdout)?)
	}
}
