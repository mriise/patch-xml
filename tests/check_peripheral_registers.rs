mod utils;

use patch_svd::get_patched_svd;
use patch_svd::output::*;

#[test]
fn test_sparse_register() {
    let patch = r#"
    device:
        peripherals:
            peripheral:
                $modify:
                    registers:
                        register:
                            name: "Register1"
                            displayName: "Register1 Display Name"
                            description: "Register1 Description"
                            addressOffset: 34
    "#;
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), patch.to_string());
    let device =
        utils::get_sparse_peripheral(Some(utils::SparsePeripheralContent::Registers(Registers {
            cluster: None,
            register: vec![Register {
                derived_from: None,
                dim: None,
                dim_increment: None,
                dim_index: None,
                dim_name: None,
                dim_array_index: None,
                name: "Register1".to_string(),
                display_name: "Register1 Display Name".to_string(),
                description: "Register1 Description".to_string(),
                alternate_group: None,
                alternate_register: None,
                address_offset: SvdConstant { value: 34 },
                size: None,
                access: None,
                protection: None,
                reset_value: None,
                reset_mask: None,
                data_type: None,
                modified_write_values: None,
                write_constraint: None,
                read_action: None,
                fields: None,
            }],
        })));
    assert_eq!(result.unwrap(), device);
}

#[test]
fn test_patched_register() {
    let patch = r#"
    device:
        peripherals:
            peripheral:
                $modify:
                    registers:
                        register:
                            derivedFrom: SomeOtherRegister
                            dim: 22
                            dimIncrement: 33
                            dimIndex: 44
                            dimName: DimName
                            dimArrayIndex:
                                headerEnumName: SomeHeaderEnumName
                                enumeratedValue:
                                    name: EnumValueName
                                    description: EnumValue Description
                                    value:
                                        value: 55
                            name: "Register2"
                            displayName: "Register2 Display Name"
                            description: "Register2 Description"
                            alternateGroup: AlternateGroupName
                            alternateRegister: AlternateRegisterName
                            addressOffset: 66
                            size: 77
                            access: writeOnce
                            protection: p
                            resetValue: 88
                            resetMask: 99
                            dataType: int64_t *
                            modifiedWriteValues: oneToToggle
                            writeConstraint: UseEnumeratedValues
                            readAction: ModifyExternal
                            fields:
                                - field:
                                    - derivedFrom: SomeOtherField
                                    - dim: 10
                                    - dimIncrement: 11
                                    - dimIndex: 12
                                    - dimName: SomeFieldDimName
                                    - dimArrayIndex:
                                        headerEnumName: SomeOtherHeaderEnumName
                                        enumeratedValue:
                                            name: SomeOtherEnumValueName
                                            description: Some Other Enum Value Description
                                            value: default
                                    - name: FieldName
                                    - description: Field Description
                                    - bitOffset: 131
                                    - bitWidth: 132
                                    - access: writeOnce
                                    - modifiedWriteValues: set
                                    - writeConstraint:
                                        Range:
                                            minimum: 14
                                            maximum: 15
                                    - readAction: ModifyExternal
                                    - +enumeratedValues:
                                        derivedFrom: SomeOtherReadEnumName
                                        name: SomeReadEnumName
                                        headerEnumName: SomeHeaderReadEnumName
                                        usage: Read
                                        enumeratedValue:
                                            name: SomeReadEnumeratedValueName
                                            description: Some Read EnumeratedValue Description
                                            value:
                                                value: 16
                                    - +enumeratedValues:
                                        derivedFrom: SomeOtherWriteEnumName
                                        name: SomeWriteEnumName
                                        headerEnumName: SomeHeaderWriteEnumName
                                        usage: Write
                                        enumeratedValue:
                                            name: SomeWriteEnumeratedValueName
                                            description: Some Write EnumeratedValue Description
                                            value:
                                                value: 17
                                - +field:
                                    name: Field2
                                    lsb: 223
                                    msb: 224
                                    enumeratedValues:
                                        usage: Read
                                        enumeratedValue:
                                            name: EnumValueName2
                                            description: Enum Value Description 2
                                            value:
                                                value: 77
                                - +field:
                                    name: Field3
                                    bitOffset: 331
                                    enumeratedValues:
                                        usage: Read
                                        enumeratedValue:
                                            name: EnumValueName3
                                            description: Enum Value Description 3
                                            value:
                                                value: 77
                                - +field:
                                    name: Field4
                                    bitRange: "\\[15:2\\]"
                                    enumeratedValues:
                                        usage: Write
                                        enumeratedValue:
                                            name: EnumValueName4
                                            description: Enum Value Description 4
                                            value:
                                                value: 77
    "#;
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), patch.to_string());
    let device =
        utils::get_sparse_peripheral(Some(utils::SparsePeripheralContent::Registers(Registers {
            cluster: None,
            register: vec![Register {
                derived_from: Some("SomeOtherRegister".to_string()),
                dim: Some(SvdConstant { value: 22 }),
                dim_increment: Some(SvdConstant { value: 33 }),
                dim_index: Some(SvdConstant { value: 44 }),
                dim_name: Some("DimName".to_string()),
                dim_array_index: Some(DimArrayIndex {
                    header_enum_name: Some("SomeHeaderEnumName".to_string()),
                    enumerated_value: vec![EnumeratedValue {
                        name: "EnumValueName".to_string(),
                        description: "EnumValue Description".to_string(),
                        value: EnumValue::Value(SvdConstant { value: 55 }),
                    }],
                }),
                name: "Register2".to_string(),
                display_name: "Register2 Display Name".to_string(),
                description: "Register2 Description".to_string(),
                alternate_group: Some("AlternateGroupName".to_string()),
                alternate_register: Some("AlternateRegisterName".to_string()),
                address_offset: SvdConstant { value: 66 },
                size: Some(SvdConstant { value: 77 }),
                access: Some(AccessType::WriteOnce),
                protection: Some(Protection::Priviledged),
                reset_value: Some(SvdConstant { value: 88 }),
                reset_mask: Some(SvdConstant { value: 99 }),
                data_type: Some(DataType::Int64TPointer),
                modified_write_values: Some(ModifiedWriteValues::OneToToggle),
                write_constraint: Some(WriteConstraint::UseEnumeratedValues),
                read_action: Some(ReadAction::ModifyExternal),
                fields: Some(Fields {
                    field: vec![
                        Field {
                            derived_from: Some("SomeOtherField".to_string()),
                            dim: Some(SvdConstant { value: 10 }),
                            dim_increment: Some(SvdConstant { value: 11 }),
                            dim_index: Some(SvdConstant { value: 12 }),
                            dim_name: Some("SomeFieldDimName".to_string()),
                            dim_array_index: Some(DimArrayIndex {
                                header_enum_name: Some("SomeOtherHeaderEnumName".to_string()),
                                enumerated_value: vec![EnumeratedValue {
                                    name: "SomeOtherEnumValueName".to_string(),
                                    description: "Some Other Enum Value Description".to_string(),
                                    value: EnumValue::Default,
                                }],
                            }),
                            name: "FieldName".to_string(),
                            description: Some("Field Description".to_string()),
                            bit_offset: Some(SvdConstant { value: 131 }),
                            bit_width: Some(SvdConstant { value: 132 }),
                            lsb: None,
                            msb: None,
                            bit_range: None,
                            access: Some(AccessType::WriteOnce),
                            modified_write_values: Some(ModifiedWriteValues::Set),
                            write_constraint: Some(WriteConstraint::Range {
                                minimum: SvdConstant { value: 14 },
                                maximum: SvdConstant { value: 15 },
                            }),
                            read_action: Some(ReadAction::ModifyExternal),
                            enumerated_values: Some(vec![
                                EnumeratedValues {
                                    derived_from: Some("SomeOtherReadEnumName".to_string()),
                                    name: Some("SomeReadEnumName".to_string()),
                                    header_enum_name: Some("SomeHeaderReadEnumName".to_string()),
                                    usage: Some(EnumeratedValuesUsage::Read),
                                    enumerated_value: vec![EnumeratedValue {
                                        name: "SomeReadEnumeratedValueName".to_string(),
                                        description: "Some Read EnumeratedValue Description"
                                            .to_string(),
                                        value: EnumValue::Value(SvdConstant { value: 16 }),
                                    }],
                                },
                                EnumeratedValues {
                                    derived_from: Some("SomeOtherWriteEnumName".to_string()),
                                    name: Some("SomeWriteEnumName".to_string()),
                                    header_enum_name: Some("SomeHeaderWriteEnumName".to_string()),
                                    usage: Some(EnumeratedValuesUsage::Write),
                                    enumerated_value: vec![EnumeratedValue {
                                        name: "SomeWriteEnumeratedValueName".to_string(),
                                        description: "Some Write EnumeratedValue Description"
                                            .to_string(),
                                        value: EnumValue::Value(SvdConstant { value: 17 }),
                                    }],
                                },
                            ]),
                        },
                        Field {
                            derived_from: None,
                            dim: None,
                            dim_increment: None,
                            dim_index: None,
                            dim_name: None,
                            dim_array_index: None,
                            name: "Field2".to_string(),
                            description: None,
                            bit_offset: None,
                            bit_width: None,
                            lsb: Some(SvdConstant { value: 223 }),
                            msb: Some(SvdConstant { value: 224 }),
                            bit_range: None,
                            access: None,
                            modified_write_values: None,
                            write_constraint: None,
                            read_action: None,
                            enumerated_values: Some(vec![EnumeratedValues {
                                derived_from: None,
                                name: None,
                                header_enum_name: None,
                                usage: Some(EnumeratedValuesUsage::Read),
                                enumerated_value: vec![EnumeratedValue {
                                    name: "EnumValueName2".to_string(),
                                    description: "Enum Value Description 2".to_string(),
                                    value: EnumValue::Value(SvdConstant { value: 77 }),
                                }],
                            }]),
                        },
                        Field {
                            derived_from: None,
                            dim: None,
                            dim_increment: None,
                            dim_index: None,
                            dim_name: None,
                            dim_array_index: None,
                            name: "Field3".to_string(),
                            description: None,
                            bit_offset: Some(SvdConstant { value: 331 }),
                            bit_width: None,
                            lsb: None,
                            msb: None,
                            bit_range: None,
                            access: None,
                            modified_write_values: None,
                            write_constraint: None,
                            read_action: None,
                            enumerated_values: Some(vec![EnumeratedValues {
                                derived_from: None,
                                name: None,
                                header_enum_name: None,
                                usage: Some(EnumeratedValuesUsage::Read),
                                enumerated_value: vec![EnumeratedValue {
                                    name: "EnumValueName3".to_string(),
                                    description: "Enum Value Description 3".to_string(),
                                    value: EnumValue::Value(SvdConstant { value: 77 }),
                                }],
                            }]),
                        },
                        Field {
                            derived_from: None,
                            dim: None,
                            dim_increment: None,
                            dim_index: None,
                            dim_name: None,
                            dim_array_index: None,
                            name: "Field4".to_string(),
                            description: None,
                            bit_offset: None,
                            bit_width: None,
                            lsb: None,
                            msb: None,
                            bit_range: Some("[15:2]".to_string()),
                            access: None,
                            modified_write_values: None,
                            write_constraint: None,
                            read_action: None,
                            enumerated_values: Some(vec![EnumeratedValues {
                                derived_from: None,
                                name: None,
                                header_enum_name: None,
                                usage: Some(EnumeratedValuesUsage::Write),
                                enumerated_value: vec![EnumeratedValue {
                                    name: "EnumValueName4".to_string(),
                                    description: "Enum Value Description 4".to_string(),
                                    value: EnumValue::Value(SvdConstant { value: 77 }),
                                }],
                            }]),
                        },
                    ],
                }),
            }],
        })));
    assert_eq!(result.unwrap(), device);
}

#[test]
fn test_register_cluster() {
    let patch = r#"
    device:
        peripherals:
            peripheral:
                $modify:
                    registers:
                        cluster:
                            derivedFrom: OtherCluster
                            dim: 11
                            dimIncrement: 22
                            dimIndex: 33
                            dimName: DimName
                            dimArrayIndex:
                                headerEnumName: HeaderEnumName
                                enumeratedValue:
                                    name: EnumeratedValueName
                                    description: Enumerated Value Description
                                    value: default
                            name: ClusterName
                            description: Cluster Description
                            alternateCluster: AlternateClusterName
                            headerStructName: ClusterHeaderStructName
                            addressOffset: 44
                            size: 55
                            access: read-only
                            protection: s
                            resetValue: 66
                            resetMask: 77
                            register:
                                dimElement: ""
                                name: Level1RegisterName
                                displayName: Level1RegisterDisplayName
                                description: Level1 Register Description
                                addressOffset: 88
                                registerProperties:
                                    size: 55
                                    access: read-only
                                    protection: secure
                                    resetValue: 66
                                    resetMask: 77
                            cluster:
                                dimElement: ""
                                name: Level2Cluster
                                addressOffset: 99
                                registerProperties:
                                    size: 55
                                    access: read-only
                                    protection: secure
                                    resetValue: 66
                                    resetMask: 77
                        register:
                            dimElement: ""
                            name: "Register1"
                            displayName: "Register1 Display Name"
                            description: "Register1 Description"
                            addressOffset: 34
                            registerProperties:
                                size: 55
                                access: read-only
                                protection: secure
                                resetValue: 66
                                resetMask: 77
    "#;
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), patch.to_string());
    let device =
        utils::get_sparse_peripheral(Some(utils::SparsePeripheralContent::Registers(Registers {
            cluster: Some(vec![Cluster {
                derived_from: Some("OtherCluster".to_string()),
                dim: Some(SvdConstant { value: 11 }),
                dim_increment: Some(SvdConstant { value: 22 }),
                dim_index: Some(SvdConstant { value: 33 }),
                dim_name: Some("DimName".to_string()),
                dim_array_index: Some(DimArrayIndex {
                    header_enum_name: Some("HeaderEnumName".to_string()),
                    enumerated_value: vec![EnumeratedValue {
                        name: "EnumeratedValueName".to_string(),
                        description: "Enumerated Value Description".to_string(),
                        value: EnumValue::Default,
                    }],
                }),
                name: "ClusterName".to_string(),
                description: Some("Cluster Description".to_string()),
                alternate_cluster: Some("AlternateClusterName".to_string()),
                header_struct_name: Some("ClusterHeaderStructName".to_string()),
                address_offset: SvdConstant { value: 44 },
                size: Some(SvdConstant { value: 55 }),
                access: Some(AccessType::ReadOnly),
                protection: Some(Protection::Secure),
                reset_value: Some(SvdConstant { value: 66 }),
                reset_mask: Some(SvdConstant { value: 77 }),
                register: Some(vec![Register {
                    derived_from: None,
                    dim: None,
                    dim_increment: None,
                    dim_index: None,
                    dim_name: None,
                    dim_array_index: None,
                    name: "Level1RegisterName".to_string(),
                    display_name: "Level1RegisterDisplayName".to_string(),
                    description: "Level1 Register Description".to_string(),
                    alternate_group: None,
                    alternate_register: None,
                    address_offset: SvdConstant { value: 88 },
                    size: None,
                    access: None,
                    protection: None,
                    reset_value: None,
                    reset_mask: None,
                    data_type: None,
                    modified_write_values: None,
                    write_constraint: None,
                    read_action: None,
                    fields: None,
                }]),
                cluster: Some(vec![Cluster {
                    derived_from: None,
                    dim: None,
                    dim_increment: None,
                    dim_index: None,
                    dim_name: None,
                    dim_array_index: None,
                    name: "Level2Cluster".to_string(),
                    description: None,
                    alternate_cluster: None,
                    header_struct_name: None,
                    address_offset: SvdConstant { value: 99 },
                    size: None,
                    access: None,
                    protection: None,
                    reset_value: None,
                    reset_mask: None,
                    register: None,
                    cluster: None,
                }]),
            }]),
            register: vec![Register {
                derived_from: None,
                dim: None,
                dim_increment: None,
                dim_index: None,
                dim_name: None,
                dim_array_index: None,
                name: "Register1".to_string(),
                display_name: "Register1 Display Name".to_string(),
                description: "Register1 Description".to_string(),
                alternate_group: None,
                alternate_register: None,
                address_offset: SvdConstant { value: 34 },
                size: None,
                access: None,
                protection: None,
                reset_value: None,
                reset_mask: None,
                data_type: None,
                modified_write_values: None,
                write_constraint: None,
                read_action: None,
                fields: None,
            }],
        })));
    assert_eq!(result.unwrap(), device);
}
