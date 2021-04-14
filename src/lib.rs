mod patch_processor;
mod patch_structure;
mod xml_structure;

use patch_processor::PatchProcessor;
use std::string::FromUtf8Error;

pub fn patch_xml(xmltree: String, patch: String) -> Result<String, String> {
    let mut processor = PatchProcessor::new(xmltree.as_str());
    match patch_structure::parse(&patch) {
        Ok(Some(patch)) => {
            processor.apply(&patch);
        }
        Ok(None) => {}
        Err(e) => return Err(format!("Error while reading patch: {}", e)),
    };
    let mut result_bytes = Vec::new();
    match processor.xml_tree.to_xmltree().write(&mut result_bytes) {
        Ok(_) => {}
        Err(msg) => return Err(format!("Error while generating XML result: {}", msg)),
    }
    String::from_utf8(result_bytes).map_err(|e| FromUtf8Error::to_string(&e))
}
