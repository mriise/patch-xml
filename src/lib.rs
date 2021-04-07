mod patch_processor;
mod patch_structure;
mod xml_structure;

use patch_processor::PatchProcessor;

pub fn patch_to_xml(xmltree: String, patch: String) -> String {
    let processor = patch_xml(xmltree, &patch);
    let mut result_bytes = Vec::new();
    match processor.xml_tree.to_xmltree().write(&mut result_bytes) {
        Ok(_) => {}
        Err(msg) => panic!("Error while writing result: {}", msg),
    }
    String::from_utf8(result_bytes).unwrap()
}

fn patch_xml(xmltree: String, patch: &String) -> PatchProcessor {
    let mut processor = PatchProcessor::new(xmltree.as_str());
    match patch_structure::parse(&patch) {
        Ok(Some(patch)) => {
            processor.apply(&patch);
        }
        Ok(None) => {}
        Err(e) => panic!("Error while reading patch: {}", e),
    };
    processor
}
