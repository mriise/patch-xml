use std::env;
use std::fs::File;
use std::io::{Read, Write};

struct InputOutput {
    xml_input_content: String,
    patch_content: String,
    result_path: String,
}

impl InputOutput {
    fn from_args(args: Vec<String>) -> Result<InputOutput, String> {
        if args.len() != 4 {
            return Err(format!(
                "usage: {} <XML-file> <patch-file (yaml)> <result-file>",
                args.get(0)
                    .ok_or("Could not get program path as first argument")?
            ));
        }

        let mut xml_file =
            File::open(args.get(1).ok_or("Could not get XML path")?).map_err(|e| e.to_string())?;
        let mut xml_input_content = String::new();
        xml_file
            .read_to_string(&mut xml_input_content)
            .map_err(|e| e.to_string())?;

        let mut patch_file = File::open(args.get(2).ok_or("Could not get patch path")?)
            .map_err(|e| e.to_string())?;
        let mut patch_content = String::new();
        patch_file
            .read_to_string(&mut patch_content)
            .map_err(|e| e.to_string())?;
        Ok(InputOutput {
            xml_input_content,
            patch_content,
            result_path: args.get(3).ok_or("Could not get result path")?.clone(),
        })
    }
}

fn main() {
    //ToDo: Implement "Import" functionality
    //ToDo: Increase test coverage to more than 95%
    let input_output = InputOutput::from_args(env::args().collect()).unwrap();
    File::create(input_output.result_path)
        .unwrap()
        .write_all(
            patch_xml::patch_xml(input_output.xml_input_content, input_output.patch_content)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::InputOutput;

    #[test]
    fn test_input_output() {
        let _ = InputOutput::from_args(vec![
            "./program".to_string(),
            "resources/testfile.txt".to_string(),
            "resources/testfile.txt".to_string(),
            "some/path.txt".to_string(),
        ])
        .unwrap();
    }
}
