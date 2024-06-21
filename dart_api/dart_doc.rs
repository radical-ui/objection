use error_stack::ResultExt;
use rand::random;
use std::{
	env::temp_dir,
	path::{Path, PathBuf},
	process::Stdio,
};
use tokio::process::Command;

use crate::{Error, Result};

/// Runs the dart documentation in a subshell. Returns the directory that the documentation was written to.
pub async fn run_dart_doc(package_path: &Path) -> Result<PathBuf> {
	let random_number = random::<u64>();
	let out_dir = temp_dir().join(format!("dart_doc_{random_number}"));

	let mut command = Command::new("dart");

	command
		.stdin(Stdio::null())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.arg("doc")
		.arg(package_path)
		.arg("--output-directory")
		.arg(&out_dir);

	let process = command.spawn().change_context(Error::ChildProcessError)?;
	let output = process.wait_with_output().await.change_context(Error::ChildProcessError)?;

	if !output.status.success() {
		Err(Error::DartDocFailed(
			String::from_utf8(output.stderr).change_context(Error::DartDocFailedWithInvalidOutput)?,
		))?
	}

	Ok(out_dir)
}
