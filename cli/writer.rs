use anyhow::{anyhow, Context, Result};
use log::info;
use reqwest::get;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::{
	fs::{create_dir_all, File},
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
		let (file, joined_path) = self.create(path).await?;
		let mut writer = BufWriter::new(file);

		writer
			.write_all(data.as_ref())
			.await
			.with_context(|| format!("failed to write {joined_path:?}"))?;

		info!("Wrote {joined_path:?}");

		Ok(())
	}

	pub async fn get_sha256(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
		let mut hasher = Sha256::new();
		let joined_path = self.directory.join(path.as_ref());
		let file = File::open(&joined_path)
			.await
			.with_context(|| format!("failed to open {joined_path:?} for reading"))?;
		let mut reader = BufReader::new(file);
		let mut buffer = [0; 1024];

		loop {
			let bytes_read = reader.read(&mut buffer).await.with_context(|| format!("failed to read {joined_path:?}"))?;
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
		let (file, joined_path) = self.create(path).await?;
		let mut writer = BufWriter::new(file);

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
					.with_context(|| format!("failed to write to {joined_path:?}"))?;
			}

			info!("Copied {}", url.path());
		} else {
			let mut response = get(url.clone()).await.with_context(|| format!("Failed to fetch {url}"))?;

			while let Some(chunk) = response.chunk().await? {
				hasher.update(&chunk[..]);

				writer
					.write_all(&chunk[..])
					.await
					.with_context(|| format!("failed to write to {joined_path:?}"))?;
			}

			info!("Downloaded {url}");
		}

		writer.flush().await?;
		Ok(hasher.finalize().to_vec())
	}

	pub fn into_file_writer(self, path: impl Into<PathBuf>) -> FileWriter {
		FileWriter {
			writer: self,
			path: path.into(),
		}
	}

	async fn create(&self, path: impl AsRef<Path>) -> Result<(File, PathBuf)> {
		let joined_path = self.directory.join(path.as_ref());

		Ok(match File::create(&joined_path).await {
			Ok(file) => (file, joined_path),
			Err(_) => {
				create_dir_all(joined_path.parent().ok_or(anyhow!("you shouldn't be writing files to /"))?).await?;

				let file = File::create(&joined_path).await.with_context(|| {
					format!(
						"failed to write {joined_path:?}, so tried to create parent directory, which succeeded.\
						But the write directly after that, to inside the new directory, failed"
					)
				})?;

				(file, joined_path)
			}
		})
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
