pub fn test_patch(xml_str: &str, patch_str: &str, expected_result: &str) {
    let result_str = patch_xml::patch_xml(xml_str.to_string(), patch_str.to_string());
    assert_eq!(
        result_str.unwrap(),
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
            expected_result
        )
        .to_string()
    );
}
