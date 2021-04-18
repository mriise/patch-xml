[![Crates.io](https://img.shields.io/crates/v/patch-xml.svg)](https://crates.io/crates/patch-xml)
[![Docs](https://docs.rs/patch-xml/badge.svg)](https://docs.rs/crate/patch-xml/)
[![Actions Status](https://github.com/VilNeo/patch-xml/workflows/Test/badge.svg)](https://github.com/VilNeo/patch-xml/actions)
[![grcov](https://img.shields.io/codecov/c/github/VilNeo/patch-xml)](https://github.com/VilNeo/patch-xml/actions)

***patch-xml* is a tool and library that reads and patches XML files.**

# Usage

*patch-xml* can be used to change XML files with a generic patch in YAML format.\
The general idea of this tool is to change parts of the XML file dynamically based on rules that are defined in the patch.

A sample code that shows the usage of this library is shown below.\
After that, an introduction to the patch rules is described in detail.

## How to use patch-xml library

Currently, patch-xml will require the unstable Rust toolchain because the ```external_doc```-feature is used.
The current state of this feature depends on [this pull request](https://github.com/rust-lang/rust/pull/83366).

```rust
use indoc::indoc;
let original_xml = r#"<element>Foo</element>"#;
let patch = indoc!(
    r#"
    element:
        Bar"#
    );
let result_xml = r#"<?xml version="1.0" encoding="UTF-8"?><element>Bar</element>"#;
// Load XML string, patch it and return the result as string
let result = patch_xml::patch_xml(original_xml.to_string(), patch.to_string()).unwrap();
assert_eq!(result, result_xml);
```
It is also possible to use *patch-xml* as command line tool.

## Patch syntax
*The syntax is almost stable and will be documented here soon...*