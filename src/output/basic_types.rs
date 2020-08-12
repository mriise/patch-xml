use core::fmt;
use regex::Regex;
use serde::{de, de::Visitor, ser, Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    pub name: String,
    pub description: String,
    pub value: EnumValue,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnumValue {
    Default,
    Value(u32),
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPropertiesGroup {
    pub size: Option<u32>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct RegExStruct {
    #[serde(skip_serializing)]
    pub regex: Regex,
}

impl PartialEq for RegExStruct {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str().to_string() == other.regex.as_str().to_string()
    }
}
impl Eq for RegExStruct {}
impl Hash for RegExStruct {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.regex.as_str().as_bytes());
        state.finish();
    }
}

impl From<String> for RegExStruct {
    fn from(regex_string: String) -> Self {
        RegExStruct {
            regex: Regex::new(format!("^{}$", regex_string).as_str()).unwrap(),
        }
    }
}

impl Into<String> for RegExStruct {
    fn into(self) -> String {
        self.regex.as_str().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    pub offset: u32,
    pub size: u32,
    pub usage: String,
    pub protection: Option<Protection>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PatchType {
    Rewrite,
    Merge,
    Add,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum ReadAction {
    Clear,
    Set,
    Modify,
    ModifyExternal,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum WriteConstraint {
    WriteAsRead,
    UseEnumeratedValues,
    Range { minimum: u32, maximum: u32 },
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum EnumeratedValuesUsage {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Protection {
    Secure,
    NonSecure,
    Priviledged,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EndianType {
    Little,
    Big,
    Selectable,
    #[allow(non_camel_case_types)]
    other,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Interrupt {
    pub name: String,
    pub description: Option<String>,
    #[serde(with = "SvdConstant")]
    pub value: u32,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimElementGroup {
    pub dim: Option<u32>,
    pub dim_increment: Option<u32>,
    pub dim_index: Option<u32>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<DimArrayIndex>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimArrayIndex {
    pub header_enum_name: Option<String>,
    pub enumerated_values: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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

pub struct SvdConstant;
impl SvdConstant {
    pub fn serialize<S>(x: &u32, y: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        y.serialize_str(format!("{:#X}", x).as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct HexVisitor;
        impl<'de> Visitor<'de> for HexVisitor {
            type Value = u32;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or a hexadecimal string starting with 0x...")
            }
            fn visit_str<E>(self, value: &str) -> Result<u32, E>
            where
                E: de::Error,
            {
                if value.to_ascii_lowercase().starts_with("0x") {
                    u32::from_str_radix(&value.to_string()[2..], 16).map_err(E::custom)
                } else {
                    u32::from_str(value).map_err(E::custom)
                }
            }
        }
        deserializer.deserialize_str(HexVisitor)
    }
}
