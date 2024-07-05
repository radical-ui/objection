mod dart_doc;
mod doc_index;
mod error;
mod fetch;
mod item;
mod package;

pub use error::*;
pub use item::*;
pub use package::*;

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
