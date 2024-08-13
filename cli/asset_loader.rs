use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, Value};
use std::collections::HashSet;
use tokio::fs::read_to_string;
use url::Url;

use crate::{
	diagnostic::{Diagnostic, DiagnosticList},
	module_loader::InfoGraph,
	writer::Writer,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawAsset {
	sha256: String,
	local_path: String,
	web_path: String,
}

#[derive(Debug)]
struct Asset {
	sha256: Vec<u8>,
	url: Url,
	web_path: String,
}

impl Asset {
	pub fn from_json(index_url: &Url, value: Value) -> Result<Asset> {
		let raw = from_value::<RawAsset>(value)?;
		let url = index_url
			.join(&raw.local_path)
			.with_context(|| format!("failed to join local path '{}' to index url '{index_url}'", raw.local_path))?;
		let sha256 = hex::decode(raw.sha256).context("sha256 is not encoded as valid hexidecimal")?;
		let web_path = normalize_web_path(&raw.web_path);

		Ok(Asset { sha256, url, web_path })
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetKind {
	Remote,
	Local,
	All,
}

#[derive(Debug, Default)]
pub struct AssetsLoader {
	indexes: Vec<Url>,
	web_paths: HashSet<String>,
	assets: Vec<Asset>,
}

impl AssetsLoader {
	pub fn register_index_url(&mut self, url: impl Into<Url>) {
		self.indexes.push(url.into());
	}

	pub async fn load(&mut self, diagnostic_list: &mut DiagnosticList) -> Result<()> {
		for index_url in self.indexes.drain(..) {
			let mut assets = match load_index(&index_url).await {
				Ok(assets) => assets,
				Err(error) => {
					println!("{}", error);
					diagnostic_list.add_error(error.context(format!("Failed to load asset index at {index_url}")));
					continue;
				}
			};

			for (asset_index, asset) in assets.drain(..).enumerate() {
				if self.web_paths.contains(&asset.web_path) {
					diagnostic_list.add(
						Diagnostic::start("Asset #")
							.text(asset_index)
							.text(" defines it's web path as ")
							.text(&asset.web_path)
							.text(", but that web path has already been registered")
							.shift()
							.text(&index_url)
							.build(),
					);
				} else {
					self.web_paths.insert(asset.web_path.clone());
					self.assets.push(asset);
				}
			}
		}

		Ok(())
	}

	pub async fn write(&self, writer: &Writer, kind: AssetKind, diagnostic_list: &mut DiagnosticList) -> Result<()> {
		let allow_all_schemes = kind == AssetKind::All;
		let allow_file_scheme = allow_all_schemes || kind == AssetKind::Local;
		let allow_other_schemes = allow_all_schemes || kind == AssetKind::Remote;

		for asset in &self.assets {
			if asset.url.scheme() == "file" && !allow_file_scheme {
				continue;
			}

			if asset.url.scheme() != "file" && !allow_other_schemes {
				continue;
			}

			if let Ok(actual_sha) = writer.get_sha256(&asset.web_path).await {
				if asset.sha256 == actual_sha {
					continue;
				}
			}

			let download_res = writer.download_file(&asset.web_path, &asset.url).await;

			let downloaded_hash = match download_res {
				Ok(hash) => hash,
				Err(error) => {
					diagnostic_list.add_error(error.context(format!("Failed to download {}", asset.url)));
					continue;
				}
			};

			if downloaded_hash != asset.sha256 {
				diagnostic_list.add(
					Diagnostic::start("After being download, the expected hash in the asset index does not match the actual hash of the file")
						.shift()
						.text(&asset.url)
						.build(),
				);
				continue;
			}
		}

		Ok(())
	}
}

fn normalize_web_path(path: &str) -> String {
	if path.starts_with("/") {
		normalize_web_path(&path[1..])
	} else if path.ends_with("/") {
		normalize_web_path(&path[..path.len() - 1])
	} else {
		format!("/{path}")
	}
}

async fn load_index(url: &Url) -> Result<Vec<Asset>> {
	let mut graph = InfoGraph::load(url).await?;
	let module = graph.modules.drain(..).nth(0).ok_or(anyhow!(
		"Expected there to be a single module (probably caused by a regression in `deno fmt`) when getting the graph for asset index"
	))?;

	if let Some(error) = module.error {
		return Err(anyhow!(error));
	}

	let local = module
		.local
		.ok_or_else(|| anyhow!("Expected a local file because there was no error. This is probably caused by a regression in `deno info`"))?;
	let json = read_to_string(&local).await.with_context(|| format!("failed to read file at {local:?}"))?;
	let value = from_str::<Value>(&json).context("Index is not valid json")?;
	let mut value_array = match value {
		Value::Array(inner) => inner,
		_ => bail!("Asset index should be a json file containing an array"),
	};

	let assets = value_array
		.drain(..)
		.enumerate()
		.map(|(index, value)| Asset::from_json(url, value).with_context(|| format!("Failed to deserialize asset #{index}")))
		.collect::<Result<Vec<_>>>()?;

	Ok(assets)
}
