mod patch_processor;
mod patch_structure;
mod xml_structure;

use std::env;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    //ToDo 1: Implement "Import" functionality
    //ToDo 2: Document code
    //ToDo 3: Increase test coverage to more than 95%
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "usage: {} <XML-file> <patch-file (yaml)> <result-file>",
            args.get(0).unwrap()
        );
        std::process::exit(1);
    }

    let mut xml_file = File::open(args.get(1).unwrap()).unwrap();
    let mut xml_content = String::new();
    xml_file.read_to_string(&mut xml_content).unwrap();
    //let mut processor = patch_processor::PatchProcessor::new(&xml_content);

    let mut yaml_file = File::open(args.get(2).unwrap()).unwrap();
    let mut yaml_content = String::new();
    yaml_file.read_to_string(&mut yaml_content).unwrap();
    File::create(args.get(3).unwrap())
        .unwrap()
        .write_all(
            patch_xml::patch_xml(xml_content, yaml_content)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use patch_xml::patch_xml;

    #[test]
    fn test_lib_call() {
        assert_eq!(
            patch_xml(
                r#"<element>Foo</element>"#.to_string(),
                indoc!(
                    r#"
                    element:
                      Bar"#
                )
                .to_string()
            )
            .unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8"?><element>Bar</element>"#.to_string()
        );
    }
    #[test]
    fn test_lib_call_with_wrong_patch() {
        assert!(patch_xml(r#"<element></element>"#.to_string(), ":".to_string()).is_err());
    }
}
