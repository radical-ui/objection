use reqwest::Url;

pub enum Item {
	Class(Class),
	Enum(Enum),
}

pub struct Class {
	pub name: String,
	pub comment: String,
	pub constructors: Vec<Constructor>,
	/// The documentation url for this class. Could be unspecified if the index contained a
	/// constructor for this class, but the class was either ommitted from the index, or has not been
	// scraped yet.
	pub doc_url: Option<Url>,
}

pub struct Constructor {
	pub name: String,
	pub comment: String,
	pub properties: Vec<Property>,
	/// The doc url for this constructor
	pub doc_url: Url,
}

pub struct Property {
	pub name: String,
	pub is_named: bool,
	pub comment: String,
	pub type_: Type,
	pub is_nullable: bool,
	/// The documentation url that belongs to this type. Some types are so self-explanitory
	/// that they do not require a doc url.
	pub type_doc_url: Option<Url>,
}

pub struct Enum {
	pub name: String,
	pub comment: String,
	pub variants: Vec<EnumVariant>,
	/// The documentation url for this enum.
	pub doc_url: Url,
}

pub struct EnumVariant {
	pub name: String,
	pub comment: String,
}

pub enum Type {
	Enum { qualified_path: String },
	Class { qualified_path: String },
	String,
	Boolean,
	Integer,
	Float,
	List { value_type: Box<Type> },
	Set { value_type: Box<Type> },
	Map { key_type: Box<Type>, value_type: Box<Type> },
	Dynamic,
	Symbol,
	Null,
}
