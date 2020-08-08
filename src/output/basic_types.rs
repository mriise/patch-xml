use crate::input::svd as input_svd;
use core::fmt;
use regex::Regex;
use serde::de::Visitor;
use serde::{de, ser, Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPropertiesGroup {
    pub size: Option<u32>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<u32>,
    pub reset_mask: Option<u32>,
}

impl RegisterPropertiesGroup {
    pub fn from(
        register_prop_group: &input_svd::RegisterPropertiesGroup,
    ) -> RegisterPropertiesGroup {
        RegisterPropertiesGroup {
            size: match &register_prop_group.size {
                None => None,
                Some(s) => Some(s.value),
            },
            access: register_prop_group.access.clone(),
            protection: register_prop_group.protection.clone(),
            reset_value: match &register_prop_group.reset_value {
                None => None,
                Some(rv) => Some(rv.value),
            },
            reset_mask: match &register_prop_group.reset_mask {
                None => None,
                Some(rm) => Some(rm.value),
            },
        }
    }
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
}

impl AddressBlock {
    pub fn from(address_block: &Option<crate::input::svd::AddressBlock>) -> Option<AddressBlock> {
        match address_block {
            None => None,
            Some(ab) => Some(AddressBlock {
                offset: ab.offset.clone(),
                size: ab.size.clone(),
                usage: ab.usage.clone(),
            }),
        }
    }
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
    #[serde(rename = "read_writeOnce")]
    ReadWriteOnce,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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
    Range(u32, u32),
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
