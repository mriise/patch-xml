use patch_svd::get_patched_svd;

#[test]
fn test_stm_svds_stm32l4x2() {
    let svd_file = std::fs::read_to_string("resources/STM32L4x2.svd").unwrap();
    let patch = r#""#;
    match get_patched_svd(svd_file, patch.to_string()) {
        Err(err) => panic!("Could not read svd file: {:?}", err),
        _ => {}
    }
}
