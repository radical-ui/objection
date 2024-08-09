use anyhow::{anyhow, Context, Result};
use log::info;
use reqwest::get;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::{
	fs::{create_dir_all, write, File},
	io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};
use url::Url;

#[derive(Debug)]
pub struct Writer {
	directory: PathBuf,
}

impl Writer {
	pub fn new(directory: impl Into<PathBuf>) -> Writer {
		Writer { directory: directory.into() }
	}

	pub async fn write_file(&self, path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> Result<()> {
		let joined_path = self.directory.join(path.as_ref());
		let path_reference = joined_path.as_path();
		let data_reference = data.as_ref();

		if let Err(_) = write(path_reference, data_reference).await {
			create_dir_all(path_reference.parent().ok_or(anyhow!("you shouldn't be writing files to /"))?).await?;
			write(path_reference, data_reference).await.context(
				"failed to write, so tried to create parent directory, which succeeded. But the write directly after that, to inside the new directory, failed",
			)?;
		}

		info!("Wrote {joined_path:?}");

		Ok(())
	}

	pub async fn get_sha256(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
		let mut hasher = Sha256::new();
		let path_reference = path.as_ref();
		let file = File::open(path_reference).await.context("failed to open {path_reference} for reading")?;
		let mut reader = BufReader::new(file);
		let mut buffer = [0; 1024];

		loop {
			let bytes_read = reader.read(&mut buffer).await.context("failed to read {path_reference}")?;
			if bytes_read == 0 {
				break;
			}

			hasher.update(&buffer[..bytes_read]);
		}

		Ok(hasher.finalize().to_vec())
	}

	/// Download `url` to `path`, returning the sha256 of the downloaded file
	pub async fn download_file(&self, path: impl AsRef<Path>, url: &Url) -> Result<Vec<u8>> {
		let mut hasher = Sha256::new();
		let path_reference = path.as_ref();
		let mut writer = BufWriter::new(
			File::create(path_reference)
				.await
				.with_context(|| format!("failed to create {path_reference:?}"))?,
		);

		if url.scheme() == "file" {
			let mut reader = BufReader::new(File::open(url.path()).await.with_context(|| format!("failed to open {}", url.path()))?);
			let mut buffer = [0; 1024];

			loop {
				let bytes_read = reader.read(&mut buffer).await.with_context(|| format!("failed to read {}", url.path()))?;
				if bytes_read == 0 {
					break;
				}

				hasher.update(&buffer[..bytes_read]);

				writer
					.write_all(&buffer[..bytes_read])
					.await
					.with_context(|| format!("failed to write to {path_reference:?}"))?;
			}
		} else {
			let mut response = get(url.clone()).await.context("Failed to fetch {url}")?;

			while let Some(chunk) = response.chunk().await? {
				hasher.update(&chunk[..]);

				writer
					.write_all(&chunk[..])
					.await
					.with_context(|| format!("failed to write to {path_reference:?}"))?;
			}
		}

		info!("Downloaded {url}");

		Ok(hasher.finalize().to_vec())
	}

	pub fn into_file_writer(self, path: impl Into<PathBuf>) -> FileWriter {
		FileWriter {
			writer: self,
			path: path.into(),
		}
	}
}

#[derive(Debug)]
pub struct FileWriter {
	writer: Writer,
	path: PathBuf,
}

impl FileWriter {
	pub async fn write(&self, data: impl AsRef<[u8]>) -> Result<()> {
		self.writer.write_file(&self.path, data).await
	}
}
