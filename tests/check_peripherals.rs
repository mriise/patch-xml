mod utils;

use patch_svd::get_patched_svd;
use patch_svd::output::*;

#[test]
fn test_sparse_peripheral() {
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), "".to_string());
    let device = utils::get_sparse_peripheral(None);
    assert_eq!(result.unwrap(), device);
}

#[test]
fn test_sparse_patched_peripheral() {
    let patch = r#"
    device:
        peripherals:
            $modify:
                peripheral:
                    name: PERIPHERAL_NAME_1
                    baseAddress: 1234
                    registerProperties: ""
    "#;
    let result = get_patched_svd(utils::SPARSE_DEVICE.to_string(), patch.to_string());
    let device = utils::get_sparse_peripheral(None);
    assert_eq!(result.unwrap(), device);
}

#[test]
fn test_general_patched_peripheral() {
    let patch = r#"
    device:
        peripherals:
            $modify:
                peripheral:
                    derivedFrom: OtherPeripheral
                    dim: 33
                    dimIncrement: 44
                    dimIndex: 55
                    dimName: DimName
                    dimArrayIndex:
                        - headerEnumName: HeaderEnumName
                        - +enumeratedValue:
                            name: NameOne
                            description: DescriptionOne
                            value:
                                value: 66
                        - +enumeratedValue:
                            name: NameTwo
                            description: DescriptionTwo
                            value: default
                    name: PERIPHERAL_NAME_2
                    version: Version2
                    description: Description2
                    alternatePeripheral: AlternatePeripheral2
                    groupName: GroupName2
                    prependToName: Prefix2
                    appendToName: Postfix2
                    headerStructName: HeaderStruct2
                    disableCondition: SomeDisableCondition2
                    baseAddress: 4321
                    size: 66
                    access: read-writeOnce
                    protection: nonSecure
                    resetValue: 77
                    resetMask: 88
                    addressBlock:
                        offset: 99
                        size: 111
                        usage: AddressBlockUsage
                        protection: secure
    "#;
    let result = get_patched_svd(utils::SPARSE_DEVICE.to_string(), patch.to_string());
    let mut device = utils::get_sparse_device();
    device.peripherals = Peripherals {
        peripheral: vec![Peripheral {
            derived_from: Some("OtherPeripheral".to_string()),
            dim: Some(SvdConstant { value: 33 }),
            dim_increment: Some(SvdConstant { value: 44 }),
            dim_index: Some(SvdConstant { value: 55 }),
            dim_name: Some("DimName".to_string()),
            dim_array_index: Some(DimArrayIndex {
                header_enum_name: Some("HeaderEnumName".to_string()),
                enumerated_value: vec![
                    EnumeratedValue {
                        name: "NameOne".to_string(),
                        description: "DescriptionOne".to_string(),
                        value: EnumValue::Value(SvdConstant { value: 66 }),
                    },
                    EnumeratedValue {
                        name: "NameTwo".to_string(),
                        description: "DescriptionTwo".to_string(),
                        value: EnumValue::Default,
                    },
                ],
            }),
            name: "PERIPHERAL_NAME_2".to_string(),
            version: Some("Version2".to_string()),
            description: Some("Description2".to_string()),
            alternate_peripheral: Some("AlternatePeripheral2".to_string()),
            group_name: Some("GroupName2".to_string()),
            prepend_to_name: Some("Prefix2".to_string()),
            append_to_name: Some("Postfix2".to_string()),
            header_struct_name: Some("HeaderStruct2".to_string()),
            disable_condition: Some("SomeDisableCondition2".to_string()),
            base_address: SvdConstant { value: 4321 },
            size: Some(SvdConstant { value: 66 }),
            access: Some(AccessType::ReadWriteOnce),
            protection: Some(Protection::NonSecure),
            reset_value: Some(SvdConstant { value: 77 }),
            reset_mask: Some(SvdConstant { value: 88 }),
            address_block: Some(AddressBlock {
                offset: SvdConstant { value: 99 },
                size: SvdConstant { value: 111 },
                usage: "AddressBlockUsage".to_string(),
                protection: Some(Protection::Secure),
            }),
            interrupt: None,
            registers: None,
        }],
    };
    assert_eq!(result.unwrap(), device);
}
