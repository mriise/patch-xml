mod utils;

use patch_svd::get_patched_svd;

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
                                    - lsb: 133
                                    - msb: 134
                                    - bitRange: "\\[15:2\\]"
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
    "#;
    let result = get_patched_svd(utils::SPARSE_PERIPHERAL.to_string(), patch.to_string());
    assert!(result.is_err());
}
