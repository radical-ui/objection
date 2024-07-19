use anyhow::Result;
use url::Url;

use crate::collect::Collection;
use std::fmt::Write;

pub fn gen_js_entry(runtime_url: &Url, engine_url: &Url, collection: &Collection) -> Result<String> {
	let mut js = String::new();

	write!(js, "import {{ start, ")?;

	for (name, _) in collection.get_component_info() {
		write!(js, "{}, ", name)?;
	}

	write!(js, " }} from '{runtime_url}'")?;
	write!(js, "\n\n")?;

	write!(js, "const renderComponent = component => {{\n")?;

	for (name, info) in collection.get_component_info() {
		write!(js, "\tif (component.type = '{}') return {}(component.def)\n", name, &info.render_name)?;
	}

	write!(js, "\tthrow new Error('Unknown component type: ' + component.type)\n")?;
	write!(js, "}}\n\n")?;
	write!(js, "const initialElement = document.getElementById('initial-state')\n")?;
	write!(js, "const initial = JSON.parse(initialElement.textContent)\n")?;

	write!(js, "start(new Url('{engine_url}'), initial, renderComponent)")?;

	Ok(js)
}
