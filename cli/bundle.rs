use anyhow::Result;
use serde::Serialize;
use serde_json::to_string;
use std::{
	collections::HashMap,
	env,
	path::{Path, PathBuf},
	process::Stdio,
};
use tokio::{io::AsyncWriteExt, process::Command};
use url::Url;

#[derive(Debug, Serialize, Default)]
struct BundleManifest {
	resolutions: HashMap<Url, HashMap<String, Url>>,
	source_files: HashMap<Url, PathBuf>,
}

#[derive(Debug, Default)]
pub struct Bundler {
	manifest: BundleManifest,
}

impl Bundler {
	pub fn register_dependency(&mut self, host: &Url, dependency: impl Into<String>, specifier: impl Into<Url>) {
		if let Some(dependencies) = self.manifest.resolutions.get_mut(host) {
			dependencies.insert(dependency.into(), specifier.into());
		}
	}

	pub fn register_source_file(&mut self, source: impl Into<Url>, file: impl Into<PathBuf>) {
		self.manifest.source_files.insert(source.into(), file.into());
	}

	pub async fn bundle(&self, entry_code: &str) -> Result<String> {
		let mut command = Command::new("deno");

		command
			.arg("run")
			.arg("bundle/main.rs")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::inherit())
			.env_clear()
			.env("PATH", env::var("PATH").unwrap());

		let mut process = command.spawn()?;
		let json = to_string(&(entry_code, &self.manifest))?;

		let mut stdin = process.stdin.take().unwrap();
		stdin.write_all(json.as_bytes());
		drop(stdin);

		let output = process.wait_with_output().await?;

		Ok(String::from_utf8(output.stdout)?)
	}
}
