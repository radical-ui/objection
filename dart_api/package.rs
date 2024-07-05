use std::collections::HashMap;

use crate::Item;

pub struct Package {
	/// Items, keyed by their qualified name
	items: HashMap<String, Item>,
}

impl Package {}
