use anyhow::Result;
use url::Url;

use crate::collect::Collection;
use std::fmt::Write;

pub fn gen_js_entry(runtime_url: &Url, engine_url: &Url, collection: &Collection) -> Result<String> {
	let mut js = String::new();

	write!(js, "import {{ start, ")?;

	for (_, info) in collection.get_component_info() {
		write!(js, "{}, ", info.render_name)?;
	}

	write!(js, " }} from '{runtime_url}'")?;
	write!(js, "\n\n")?;

	write!(js, "window.objectionSelectComponentRenderer = component => {{\n")?;

	for (name, info) in collection.get_component_info() {
		write!(
			js,
			"\tif (component.type === '{}') return {{ func: {}, params: component.def }}\n",
			name, &info.render_name
		)?;
	}

	write!(js, "\tthrow new Error('Unknown component type: ' + component.type)\n")?;
	write!(js, "}}\n\n")?;
	write!(js, "const initialElement = document.getElementById('initial-state')\n")?;
	write!(js, "const initial = JSON.parse(initialElement.textContent)\n")?;
	write!(js, "window.objectionEndpoint = new URL('{engine_url}')\n")?;

	write!(js, "start(initial)")?;

	Ok(js)
}
