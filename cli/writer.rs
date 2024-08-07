use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use tokio::fs::{create_dir_all, write};

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

		Ok(())
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
