use std::collections::HashMap;

pub struct Package {
	/// Items, keyed by their qualified name
	items: HashMap<String, Item>,
}

pub struct Item {}
