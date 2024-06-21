use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Child process threw an error")]
	ChildProcessError,
	#[error("`dart doc` exited with a non-zero exit code.\n{0}")]
	DartDocFailed(String),
	#[error("`dart doc` exited with a non-zero exit code and it's stderr was not valid utf8")]
	DartDocFailedWithInvalidOutput,
	#[error("`dart doc` published an output that was not expected")]
	InvalidDartDocOuput,
	#[error("Something very odd happened")]
	HighlyOdd,
	#[error("Failed to read from filesystem")]
	FailedToReadFile,
	#[error("Failed to fetch")]
	FailedToFetch,
	#[error("Resource not found")]
	ResourceNotFound,
}

pub type Result<T> = error_stack::Result<T, Error>;
