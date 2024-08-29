use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {}

impl Space {
	pub fn new() -> Space {
		Space {}
	}
}
