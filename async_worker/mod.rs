use log::error;
use std::fmt::Debug;
use thiserror::Error;

mod handle;
mod queue;
mod worker;

pub use handle::{DropReason, NoopHandle, SendResult, WorkerHandle};
pub use queue::{Queue, QueueBuilder};
pub use worker::{PeerRequest, Worker};

#[derive(Debug, Error)]
pub enum Error {
	/// Thrown when a particular worker has reached it's capcity of pending operations.
	#[error("Worker is at capacity, meaning that the number of pending operations has reached it's configued limit. This limit can be adjusted via QueueBuilder::max_length")]
	WorkerAtCapacity,

	/// Thrown when there is no worker available to complete the requested operation
	#[error("No worker exists for the given id")]
	NoWorker,

	/// Thrown when a newer poll or handle has been pased to a worker that already has an ongoing poll or handle. The ongoing handle will be ceeded to the newer one
	/// and give this error.
	#[error("This operation has been ceeded in favor of a newer operation")]
	Ceeded,

	/// Thrown when a worker is terminating while waiting, usually while waiting for a poll to complete
	#[error("The worker was terminated while this operation was in progress")]
	WorkerTerminated,

	/// Thrown when an operation timed out. Only thrown in the `Queue::*_while` methods: `Queue::poll_while` and `Queue::poll_many_while`
	#[error("This operation timed out")]
	Timeout,
}

type Result<T> = std::result::Result<T, Error>;
