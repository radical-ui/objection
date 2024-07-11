use svelte_toolbox::{Action, TextInputUpdate, UpdateAction, Window};
use schemars::schema_for;
use serde_json::to_string_pretty;

fn main() {
	let schema = schema_for!((Window, UpdateAction, Action, TextInputUpdate));

	println!("{}", to_string_pretty(&schema).unwrap())
}
