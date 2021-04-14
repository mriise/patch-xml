mod utils;

use patch_svd::get_patched_svd;
use patch_svd::output::*;

#[test]
fn test_interrupts() {
    let patch = r#"
    device:
        peripherals:
            peripheral:
                $modify:
                    - +interrupt:
                        name: InterruptName1
                        description: "Interrupt description one"
                        value: 44
                    - +interrupt:
                        name: InterruptName2
                        description: "Interrupt description two"
                        value: 55
    "#;
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), patch.to_string());
    let device =
        utils::get_sparse_peripheral(Some(utils::SparsePeripheralContent::Interrupts(vec![
            Interrupt {
                name: "InterruptName1".to_string(),
                description: Some("Interrupt description one".to_string()),
                value: SvdConstant { value: 44 },
            },
            Interrupt {
                name: "InterruptName2".to_string(),
                description: Some("Interrupt description two".to_string()),
                value: SvdConstant { value: 55 },
            },
        ])));
    assert_eq!(result.unwrap(), device);
}
