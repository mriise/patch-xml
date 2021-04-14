pub mod output;

pub fn get_patched_svd(
    svd_content: String,
    patch_content: String,
) -> Result<output::Device, serde_xml_rs::Error> {
    let patched_svd = patch_xml::patch_xml(svd_content, patch_content);
    serde_xml_rs::from_reader(patched_svd.unwrap().as_bytes())
}
