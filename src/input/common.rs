use core::fmt;
use regex::Regex;
use serde::de::Visitor;
use serde::{de, ser, Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

pub struct HexSerde;
impl HexSerde {
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
