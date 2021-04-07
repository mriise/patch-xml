mod utils;

use patch_svd::get_patched_svd;
use patch_svd::output::*;

#[test]
fn test_unpatched_sparse_device() {
    let result = get_patched_svd(utils::SPARSE_DEVICE.to_string(), "".to_string());
    assert_eq!(result.unwrap(), utils::get_sparse_device());
}

#[test]
fn test_wrong_svd_constant() {
    use serde::Deserialize;
    #[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
    #[serde(rename_all = "camelCase")]
    struct Tmp {
        value: SvdConstant,
    }
    let result: Result<Tmp, serde_xml_rs::Error> =
        serde_xml_rs::from_str(&"<root><value>Hello World</value></root>".to_string());
    match result {
        Ok(result) => panic!(
            "\"No Number\" should not be convertible to an SvdConstant. Result is: {:?}",
            result
        ),
        Err(err) => match &err {
            serde_xml_rs::Error::Custom { .. } => {}
            _ => panic!("Expected CustomError, got {:?} instead.", err),
        },
    }
}

#[test]
fn test_patched_sparse_device() {
    let patch = r#"
    device:
        $modify:
            schemaVersion: "2.0.0"
            vendor: "MyFavoriteVendor"
            vendorId: "0x1234"
            name: "UpdatedUcName"
            series: "Some series"
            version: "3.0.0"
            description: "Some other description"
            licenseText: |-
              Some
              license
              text
            cpu:
                name: "Other CPU name"
                revision: "r5"
                endian: big
                mpuPresent: false
                fpuPresent: false
                fpuDP: true
                dspPresent: false
                icachePresent: true
                dcachePresent: false
                itcmPresent: true
                dtcmPresent: false
                vtorPresent: true
                nvicPrioBits: 32
                vendorSystickConfig: false
                deviceNumInterrupts: 32
                sauNumRegions: 256
                sauRegionsConfig:
                    enabled: true
                    protectionWhenDisabled: priviledged
                    regions:
                        - +region:
                            enabled: true
                            name: "Region 1"
                            base: 27
                            limit: 72
                            access: "RW"
                        - +region:
                            enabled: false
                            name: "Region 2"
                            base: 29
                            limit: 42
                            access: "R"
            headerSystemFilename: "Some filename"
            headerDefinitionsPrefix: "Some definition prefix"
            addressUnitBits: '#1101'
            width: '0xFF'
            size: 32
            access: read-only
            protection: nonSecure
            resetValue: 0
            resetMask: 255
            peripherals:
                peripheral:
                    name: PeripheralName
                    baseAddress: 77
    "#;
    let result = get_patched_svd(utils::SPARSE_DEVICE.to_string(), patch.to_string());
    assert_eq!(
        result.unwrap(),
        Device {
            schema_version: "2.0.0".to_string(),
            vendor: Some("MyFavoriteVendor".to_string()),
            vendor_id: Some("0x1234".to_string()),
            name: "UpdatedUcName".to_string(),
            series: Some("Some series".to_string()),
            version: "3.0.0".to_string(),
            description: "Some other description".to_string(),
            license_text: Some("Some\nlicense\ntext".to_string()),
            cpu: Cpu {
                name: "Other CPU name".to_string(),
                revision: "r5".to_string(),
                endian: EndianType::Big,
                mpu_present: false,
                fpu_present: false,
                fpu_d_p: Some(true),
                dsp_present: Some(false),
                icache_present: Some(true),
                dcache_present: Some(false),
                itcm_present: Some(true),
                dtcm_present: Some(false),
                vtor_present: Some(true),
                nvic_prio_bits: SvdConstant { value: 32 },
                vendor_systick_config: false,
                device_num_interrupts: Some(SvdConstant { value: 32 }),
                sau_num_regions: Some(SvdConstant { value: 256 }),
                sau_regions_config: Some(SauRegionsConfigType {
                    enabled: Some(true),
                    protection_when_disabled: Some(Protection::Priviledged),
                    regions: Some(SauRegionsType {
                        region: vec![
                            SauRegionType {
                                enabled: Some(true),
                                name: Some("Region 1".to_string()),
                                base: SvdConstant { value: 27 },
                                limit: SvdConstant { value: 72 },
                                access: "RW".to_string()
                            },
                            SauRegionType {
                                enabled: Some(false),
                                name: Some("Region 2".to_string()),
                                base: SvdConstant { value: 29 },
                                limit: SvdConstant { value: 42 },
                                access: "R".to_string()
                            }
                        ]
                    })
                })
            },
            header_system_filename: Some("Some filename".to_string()),
            header_definitions_prefix: Some("Some definition prefix".to_string()),
            address_unit_bits: SvdConstant { value: 0b1101 },
            width: SvdConstant { value: 0xFF },
            size: Some(SvdConstant { value: 32 }),
            access: Some(AccessType::ReadOnly),
            protection: Some(Protection::NonSecure),
            reset_value: Some(SvdConstant { value: 0 }),
            reset_mask: Some(SvdConstant { value: 255 }),
            peripherals: Peripherals {
                peripheral: vec![Peripheral {
                    derived_from: None,
                    dim: None,
                    dim_increment: None,
                    dim_index: None,
                    dim_name: None,
                    dim_array_index: None,
                    name: "PeripheralName".to_string(),
                    version: None,
                    description: None,
                    alternate_peripheral: None,
                    group_name: None,
                    prepend_to_name: None,
                    append_to_name: None,
                    header_struct_name: None,
                    disable_condition: None,
                    base_address: SvdConstant { value: 77 },
                    size: None,
                    access: None,
                    protection: None,
                    reset_value: None,
                    reset_mask: None,
                    address_block: None,
                    interrupt: None,
                    registers: None
                }]
            }
        }
    );
}
