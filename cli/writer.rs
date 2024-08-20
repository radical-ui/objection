use anyhow::{anyhow, Context, Result};
use bytes::{Bytes, BytesMut};
use futures::{pin_mut, stream::unfold, Stream, TryStreamExt};
use log::info;
use reqwest::get;
use sha2::{Digest, Sha256};
use std::{
	fs::File,
	io::{BufReader, BufWriter, Read, Write},
	path::{Path, PathBuf},
};
use tokio::fs::create_dir_all;
use url::Url;

#[derive(Debug)]
pub struct Writer {
	directory: PathBuf,
}

impl Writer {
	pub fn new(directory: impl Into<PathBuf>) -> Writer {
		Writer { directory: directory.into() }
	}

	#[deprecated = "Use writer.file(...).write(...) instead"]
	pub async fn write_file(&self, path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> Result<()> {
		let joined_path = self.directory.join(path);
		let file = create(&joined_path)?;
		let mut writer = BufWriter::new(file);

		writer.write_all(data.as_ref()).with_context(|| format!("failed to write {joined_path:?}"))?;

		info!("Wrote {joined_path:?}");

		Ok(())
	}

	pub async fn get_sha256(&self, path: impl AsRef<Path>) -> Result<Vec<u8>> {
		let mut hasher = Sha256::new();
		let joined_path = self.directory.join(path.as_ref());
		let file = File::open(&joined_path).with_context(|| format!("failed to open {joined_path:?} for reading"))?;
		let mut reader = BufReader::new(file);
		let mut buffer = [0; 1024];

		loop {
			let bytes_read = reader.read(&mut buffer).with_context(|| format!("failed to read {joined_path:?}"))?;
			if bytes_read == 0 {
				break;
			}

			hasher.update(&buffer[..bytes_read]);
		}

		Ok(hasher.finalize().to_vec())
	}

	/// Download `url` to `path`, returning the sha256 of the downloaded file
	#[deprecated = "Use writer.file(...).write(reqwest::get(...).await.to_stream()).await instead"]
	pub async fn download_file(&self, path: impl AsRef<Path>, url: &Url) -> Result<Vec<u8>> {
		let joined_path = self.directory.join(path);
		let mut hasher = Sha256::new();
		let file = create(&joined_path)?;
		let mut writer = BufWriter::new(file);

		if url.scheme() == "file" {
			let mut reader = BufReader::new(File::open(url.path()).with_context(|| format!("failed to open {}", url.path()))?);
			let mut buffer = [0; 1024];

			loop {
				let bytes_read = reader.read(&mut buffer).with_context(|| format!("failed to read {}", url.path()))?;
				if bytes_read == 0 {
					break;
				}

				hasher.update(&buffer[..bytes_read]);

				writer
					.write_all(&buffer[..bytes_read])
					.with_context(|| format!("failed to write to {joined_path:?}"))?;
			}

			info!("Copied {}", url.path());
		} else {
			let mut response = get(url.clone()).await.with_context(|| format!("Failed to fetch {url}"))?;

			while let Some(chunk) = response.chunk().await? {
				hasher.update(&chunk[..]);

				writer.write_all(&chunk[..]).with_context(|| format!("failed to write to {joined_path:?}"))?;
			}

			info!("Downloaded {url}");
		}

		writer.flush()?;
		Ok(hasher.finalize().to_vec())
	}

	pub fn get_full_path(&self, path: impl AsRef<Path>) -> PathBuf {
		self.directory.join(path.as_ref())
	}

	#[deprecated = "Use Writer::file(...) instead"]
	pub fn into_file_writer(self, path: impl Into<PathBuf>) -> FileWriter {
		self.file(path.into())
	}

	pub fn file(&self, path: impl AsRef<Path>) -> FileWriter {
		FileWriter {
			path: self.directory.join(path),
		}
	}

	pub fn sub_dir(&self, path: impl AsRef<Path>) -> Writer {
		Writer {
			directory: self.directory.join(path),
		}
	}
}

#[derive(Debug)]
pub struct FileWriter {
	path: PathBuf,
}

impl FileWriter {
	pub async fn write(&self, stream: impl Stream<Item = Result<Bytes>>) -> Result<()> {
		let file = create(&self.path)?;
		let mut writer = BufWriter::new(file);

		pin_mut!(stream);

		while let Some(bytes) = stream.try_next().await? {
			writer.write_all(&bytes)?;
		}

		Ok(())
	}

	pub fn read(&self) -> impl Stream<Item = Result<Bytes>> {
		enum State {
			ShouldOpen(PathBuf),
			Reading(BufReader<File>),
			Done,
		}

		unfold(State::ShouldOpen(self.path.clone()), |state| async {
			let mut reader = match state {
				State::ShouldOpen(path) => {
					let file = match File::open(&path) {
						Ok(file) => file,
						Err(error) => return Some((Err(anyhow!(error).context(format!("Failed to open {path:?} for reading"))), State::Done)),
					};

					BufReader::new(file)
				}
				State::Reading(reader) => reader,
				State::Done => return None,
			};

			let mut buffer = BytesMut::zeroed(4096);
			let bytes_read = match reader.read(&mut buffer) {
				Ok(bytes_read) => bytes_read,
				Err(error) => return Some((Err(anyhow!(error).context("Failed to read bytes from an already opened file")), State::Done)),
			};

			if bytes_read == 0 {
				return None;
			}

			buffer.truncate(bytes_read);

			Some((Ok(buffer.freeze()), State::Reading(reader)))
		})
	}

	pub fn get_sha256(&self) -> Result<Vec<u8>> {
		let mut hasher = Sha256::new();
		let file = File::open(&self.path).with_context(|| format!("failed to open {:?} for reading", self.path))?;
		let mut reader = BufReader::new(file);
		let mut buffer = [0; 1024];

		loop {
			let bytes_read = reader.read(&mut buffer).with_context(|| format!("failed to read {:?}", self.path))?;
			if bytes_read == 0 {
				break;
			}

			hasher.update(&buffer[..bytes_read]);
		}

		Ok(hasher.finalize().to_vec())
	}
}

fn create(path: impl AsRef<Path>) -> Result<File> {
	Ok(match File::create(path.as_ref()) {
		Ok(file) => file,
		Err(_) => {
			create_dir_all(path.as_ref().parent().ok_or(anyhow!("you shouldn't be writing files to /"))?);

			let file = File::create(path.as_ref()).with_context(|| {
				format!(
					"failed to write {:?}, so tried to create parent directory, which succeeded.\
					But the write directly after that, to inside the new directory, failed",
					path.as_ref()
				)
			})?;

			file
		}
	})
}
