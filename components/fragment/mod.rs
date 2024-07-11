use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Component;

/// A "nothing" component. Renders nothing.
///
/// **Example**
///
/// ```rust
/// Fragment
/// ```
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Fragment;

impl From<()> for Component {
	fn from(_: ()) -> Self {
		Component::Fragment(Fragment)
	}
}
