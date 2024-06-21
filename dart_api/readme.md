# dart_api

Given a dart package, parse it into a graph of public items and their dependencies.

## Usage

```rust
use dart_api::Package;

let mut package = Package::parse_from_directory("a_directory_with_dart_code");

// The roots are the public classes of public files in the package
package.get_roots(); // &[Item { ... }, Item { ... }]

// Release all roots except the first. All unnecessary dependencies will be dropped.
package.reduce_roots(package.get_roots().get(0).unwrap()); 

// Get all the classes graph. The current roots and all their dependencies
for item in package.items() {
  class.get_name(); // Something like "Foo"
  class.get_path(); // Something like "package.some_file.Foo"
  class.get_comment(); // The doc comment (if any) that was placed on this class

  match item {
    Item::Class(class) => {
      for property in class.get_properties() {
        property.get_name(); // Something like "name"
        property.is_named(); // If flase, this argument is positional
        property.get_comment(); // The doc comment (if any) that was placed on this property
        property.get_type(); // Item { .. } 
      }
    }
    Item::Enum(enum_) => {
      for variant in enum_.get_variants() {
        variant.get_name();
        variant.get_comment();
      }
    }
}
```

## Internals

This crate works by running `dart doc` on the dart package, which, in addition to an html file for every
item, produces an `index.json` file.

This is an outline of the logic used:

1. The index is parsed.
2. The html page of all relevant items is scraped.
3. If the html page refreneces another item of relevance, check if that item is in the index
  - If it is, do nothing. If it hasn't already been scraped, it will be scraped soon.
  - If it is not, it is probably a doc link to an external package.
    - Using this url, walk up the path until an index.json is found
    - Using this index, repeat step 1.
    - Now, the item should exist.
