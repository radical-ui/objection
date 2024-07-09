mod dart_doc;
mod error;
mod fetch;
mod graph;
mod item;
mod scrape;

pub use error::*;
pub use graph::*;
pub use item::*;

/// Slice a name into a base and last component
///
/// ```ignore
/// slice_on_last_dot("hello.there.friend"); // (Some("hello.there"), "friend")
/// slice_on_last_dot("hi"); // (None, "hi")
/// ```
fn slice_on_last_dot(base: &str) -> (Option<&str>, &str) {
	let last_dot_index = match base.find('.') {
		Some(index) => index,
		None => return (None, base),
	};

	(Some(&base[..last_dot_index]), &base[last_dot_index + 1..])
}
