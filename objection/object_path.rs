use http::header::ValueDrain;

#[derive(Debug)]
enum Component {
	Literal(String),
	Dynamic,
}

#[derive(Debug)]
pub struct ObjectPath {
	components: Vec<Component>,
}

impl ObjectPath {
	pub fn new(path: &str) -> ObjectPath {
		let components = path
			.split("/")
			.map(|item| {
				if item == "*" {
					Component::Dynamic
				} else {
					Component::Literal(item.to_string())
				}
			})
			.collect();

		ObjectPath { components }
	}

	pub fn does_match(&self, path_segments: &[&str]) -> Option<Vec<String>> {
		if self.components.len() != path_segments.len() {
			return None;
		}

		let mut dynamic_parts = Vec::new();

		for (index, component) in self.components.iter().enumerate() {
			let segment = path_segments[index];

			match component {
				Component::Literal(expected) => {
					if expected != segment {
						return None;
					}
				}
				Component::Dynamic => dynamic_parts.push(segment.to_string()),
			}
		}

		return Some(dynamic_parts);
	}
}

impl From<String> for ObjectPath {
	fn from(value: String) -> Self {
		ObjectPath::new(&value)
	}
}

impl From<&str> for ObjectPath {
	fn from(value: &str) -> Self {
		ObjectPath::new(value)
	}
}

#[derive(Debug)]
pub struct ObjectPathIndex<T> {
	paths: Vec<(ObjectPath, T)>,
}

impl<T: Clone> ObjectPathIndex<T> {
	pub fn new() -> ObjectPathIndex<T> {
		ObjectPathIndex { paths: Vec::new() }
	}

	pub fn define(&mut self, path: impl Into<ObjectPath>, value: T) {
		self.paths.push((path.into(), value))
	}

	pub fn find_match(&self, id: &str) -> Option<ObjectIndexMatch<T>> {
		let segments = id.split("/").collect::<Vec<_>>();

		for (path, value) in &self.paths {
			if let Some(dynamic_parts) = path.does_match(&segments) {
				return Some(ObjectIndexMatch {
					dynamic_parts,
					value: value.clone(),
				});
			}
		}

		None
	}
}

#[derive(Debug)]
pub struct ObjectIndexMatch<T> {
	pub dynamic_parts: Vec<String>,
	pub value: T,
}
