mod patch_processor;
mod patch_structure;
mod xml_structure;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    //ToDo  1: Program the binary framework: Require three arguments, read the first as xml and the second as yaml, apply the yaml (identity) on the xml and write the result to the path that was given as third argument
    //ToDo  2: Implement framework for command pattern parser: IncomingString->(optionally Command+)String
    //ToDo  3: Implement framework for capture std::fs::File;erencing: [...]+PathInXMLTree(incl. Captures)->String
    //ToDo  4: Overwrite
    //ToDo  5: Create new entries
    //ToDo  6: Attributes
    //ToDo  7: Removing
    //ToDo  8: Paths
    //ToDo  9: Regular expressions
    //ToDo 10: Moving and copying
    //ToDo 11: Self referencing
    //ToDo 12: Filtering
    //ToDo 13: Importing
    //ToDo 14: Document code
    //ToDo 15: Test code

    //ToDo: Order:
    // 1. Remove or overwrite
    // 2. Attributes
    // 3. Create new entries
    // 4. Moving and copying
    //ToDo: Ad-Hoc:
    // 1. Paths
    // 2. Regular expressions
    // 3. Self referencing
    // 4. Filtering
    // 5. Importing

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
    let mut processor = patch_processor::PatchProcessor::new(&xml_content);

    let mut yaml_file = File::open(args.get(2).unwrap()).unwrap();
    let mut yaml_content = String::new();
    yaml_file.read_to_string(&mut yaml_content).unwrap();
    match patch_structure::parse(&yaml_content) {
        Ok(Some(patch)) => {
            processor.apply(&patch);
            processor.write_result(args.get(3).unwrap());
        }
        Ok(None) => {
            println!("Empty patch. No changes applied.");
            processor.write_result(args.get(3).unwrap());
        }
        Err(msg) => panic!("Error while parsing patch file: {}", msg),
    }

    println!("XML tree successfully patched!");
}
