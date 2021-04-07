use serde::de::Error;
use serde::{de, Deserialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    pub name: String,
    pub description: String,
    pub value: EnumValue,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnumValue {
    Default,
    Value(SvdConstant),
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    pub offset: SvdConstant,
    pub size: SvdConstant,
    pub usage: String,
    pub protection: Option<Protection>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AccessType {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    #[serde(rename = "writeOnce")]
    WriteOnce,
    #[serde(rename = "read-writeOnce")]
    ReadWriteOnce,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ModifiedWriteValues {
    OneToClear,
    OneToSet,
    OneToToggle,
    ZeroToClear,
    ZeroToSet,
    ZeroToToggle,
    Clear,
    Set,
    Modify,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum ReadAction {
    Clear,
    Set,
    Modify,
    ModifyExternal,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum WriteConstraint {
    WriteAsRead,
    UseEnumeratedValues,
    Range {
        minimum: SvdConstant,
        maximum: SvdConstant,
    },
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum EnumeratedValuesUsage {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Protection {
    Secure,
    NonSecure,
    Priviledged,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum CpuNameType {
    CM0,
    CM0PLUS,
    CM1,
    SC000,
    CM23,
    CM3,
    CM33,
    CM35P,
    SC300,
    CM4,
    CM7,
    CA5,
    CA7,
    CA8,
    CA9,
    CA15,
    CA17,
    CA53,
    CA57,
    CA72,
    #[allow(non_camel_case_types)]
    other,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EndianType {
    Little,
    Big,
    Selectable,
    #[allow(non_camel_case_types)]
    other,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    pub value: SvdConstant,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimArrayIndex {
    pub header_enum_name: Option<String>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    Uint8T,
    Uint16T,
    Uint32T,
    Uint64T,
    Int8T,
    Int16T,
    Int32T,
    Int64T,
    #[serde(rename = "uint8_t *")]
    Uint8TPointer,
    #[serde(rename = "uint16_t *")]
    Uint16TPointer,
    #[serde(rename = "uint32_t *")]
    Uint32TPointer,
    #[serde(rename = "uint64_t *")]
    Uint64TPointer,
    #[serde(rename = "int8_t *")]
    Int8TPointer,
    #[serde(rename = "int16_t *")]
    Int16TPointer,
    #[serde(rename = "int32_t *")]
    Int32TPointer,
    #[serde(rename = "int64_t *")]
    Int64TPointer,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SvdConstant {
    pub value: u32,
}

impl<'de> de::Deserialize<'de> for SvdConstant {
    fn deserialize<D>(deserializer: D) -> Result<SvdConstant, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = if value.to_ascii_lowercase().starts_with("#") {
            u32::from_str_radix(&value.to_string()[1..], 2).map_err(D::Error::custom)
        } else if value.to_ascii_lowercase().starts_with("0x") {
            u32::from_str_radix(&value.to_string()[2..], 16).map_err(D::Error::custom)
        } else {
            u32::from_str(value.as_str()).map_err(D::Error::custom)
        };
        match result {
            Ok(result) => Ok(SvdConstant { value: result }),
            Err(err) => Err(err),
        }
    }
}
