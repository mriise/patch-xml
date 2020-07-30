use crate::input::svd as input_svd;
use core::fmt;
use regex::Regex;
use serde::de::Visitor;
use serde::{de, ser, Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use crate::input::svd::{AddressBlock, Endian};
use itertools::Itertools;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Svd {
    pub device: Device,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub name: String,
    pub version: String,
    pub description: String,
    pub address_unit_bits: u32,
    pub width: u32,
    #[serde(with = "HexSerde")]
    pub size: u32,
    #[serde(with = "HexSerde")]
    pub reset_value: u32,
    #[serde(with = "HexSerde")]
    pub reset_mask: u32,
    pub cpu: Cpu,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: Endian,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub nvic_prio_bits: u32,
    pub vendor_systick_config: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Peripheral {
    #[serde(rename_all = "camelCase")]
    Content {
        name: String,
        #[serde(with = "HexSerde")]
        base_address: u32,
        address_block: Option<AddressBlock>,
        version: Option<String>,
        description: String,
        group_name: Option<String>,
        prepend_to_name: Option<String>,
        append_to_name: Option<String>,
        disable_condition: Option<String>,
        registers: Vec<Register>,
        atomic_registers: Vec<Register>,
    },
    #[serde(rename_all = "camelCase")]
    Derived {
        name: String,
        #[serde(with = "HexSerde")]
        base_address: u32,
        derived_from: String,
    },
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub name: String,
    #[serde(with = "HexSerde")]
    pub size: u32,
    #[serde(with = "HexSerde")]
    pub reset_value: u32,
    //Supported by yaml-config
    pub display_name: String,
    pub description: String,
    pub access: Option<AccessType>,

    pub alternate_group: Option<String>,
    #[serde(with = "HexSerde")]
    pub address_offset: u32,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub read_fields: Vec<Field>,
    pub write_fields: Vec<Field>,
    pub read_write_fields: Vec<Field>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub mask: u32,
    pub description: Option<String>,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub field_type: FieldType,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnumeratedValues {
    Content {
        name: String,
        usage: Option<EnumeratedValuesUsage>,
        enumerated_value: Vec<EnumeratedValue>,
    },
    Derived {
        derived_from: String,
    },
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FieldType {
    Raw(String),
    Enum(EnumeratedValues),
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnumeratedValue {
    Valued {
        name: String,
        description: String,
        #[serde(with = "HexSerde")]
        value: u32,
    },
    Default {
        name: String,
        description: String,
    },
}

impl Svd {
    pub fn from(svd: &input_svd::Svd) -> Svd {
        Svd {
            device: Device {
                name: svd.device.name.clone(),
                version: svd.device.version.clone(),
                description: svd.device.description.clone(),
                address_unit_bits: svd.device.address_unit_bits.clone(),
                width: svd.device.width.clone(),
                size: svd.device.size.clone(),
                reset_value: svd.device.reset_value.clone(),
                reset_mask: svd.device.reset_mask.clone(),
                cpu: Cpu {
                    name: svd.device.cpu.name.clone(),
                    revision: svd.device.cpu.revision.clone(),
                    endian: svd.device.cpu.endian.clone(),
                    mpu_present: svd.device.cpu.mpu_present.clone(),
                    fpu_present: svd.device.cpu.fpu_present.clone(),
                    nvic_prio_bits: svd.device.cpu.nvic_prio_bits.clone(),
                    vendor_systick_config: svd.device.cpu.vendor_systick_config.clone(),
                },
                peripherals: svd
                    .device
                    .peripherals
                    .peripheral
                    .iter()
                    .sorted_by(|p1, p2| p1.base_address.cmp(&p2.base_address))
                    .map(|p| Peripheral::from(&p))
                    .collect(),
            },
        }
    }
}

impl Peripheral {
    pub fn from(peripheral: &input_svd::Peripheral) -> Peripheral {
        if peripheral.derived_from.is_some() {
            Peripheral::Derived {
                derived_from: peripheral.derived_from.as_ref().unwrap().clone(),
                name: peripheral.name.clone(),
                base_address: peripheral.base_address.clone(),
            }
        } else {
            Peripheral::Content {
                name: peripheral.name.clone(),
                base_address: peripheral.base_address.clone(),
                address_block: peripheral.address_block.clone(),
                version: peripheral.version.clone(),
                description: peripheral.description.as_ref().unwrap().clone(),
                group_name: peripheral.group_name.clone(),
                prepend_to_name: peripheral.prepend_to_name.clone(),
                append_to_name: peripheral.append_to_name.clone(),
                disable_condition: peripheral.disable_condition.clone(),
                registers: peripheral
                    .registers
                    .as_ref()
                    .unwrap()
                    .register
                    .iter()
                    .map(|r| Register::from(&r))
                    .sorted_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset))
                    .collect(),
                atomic_registers: vec![],
            }
        }
    }
}

impl Register {
    pub fn from(patch_svd_register: &input_svd::Register) -> Register {
        let access = patch_svd_register.access.clone();
        Register {
            name: patch_svd_register.name.clone(),
            size: patch_svd_register.size.clone(),
            reset_value: patch_svd_register.reset_value.clone(),
            display_name: patch_svd_register.display_name.clone(),
            description: patch_svd_register.description.clone(),
            access: access.clone(),
            alternate_group: patch_svd_register.alternate_group.clone(),
            address_offset: patch_svd_register.address_offset.clone(),
            modified_write_values: patch_svd_register.modified_write_values.clone(),
            write_constraint: patch_svd_register.write_constraint.clone(),
            read_action: patch_svd_register.read_action.clone(),
            read_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
                .collect(),
            write_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::WriteOnly
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
                .collect(),
            read_write_fields: patch_svd_register
                .fields
                .field
                .iter()
                .filter(|f| {
                    Register::prioritize_access_type(&access, &f.access) == AccessType::ReadWrite
                })
                .sorted_by(|f1, f2| f1.bit_offset.cmp(&f2.bit_offset))
                .map(|f| Field::from(&f))
                .collect(),
        }
    }
    fn prioritize_access_type(
        register_access_type: &Option<AccessType>,
        field_access_type: &Option<AccessType>,
    ) -> AccessType {
        if field_access_type.is_some() {
            field_access_type.as_ref().unwrap().clone()
        } else if register_access_type.is_some() {
            register_access_type.as_ref().unwrap().clone()
        } else {
            AccessType::ReadWrite
        }
    }
}

impl Field {
    pub fn from(field: &input_svd::Field) -> Field {
        Field {
            name: field.name.clone(),
            mask: ((((1 as usize) << field.bit_width as usize) - 1) << field.bit_offset) as u32,
            description: field.description.clone(),
            access: field.access.clone(),
            modified_write_values: field.modified_write_values.clone(),
            write_constraint: field.write_constraint.clone(),
            read_action: field.read_action.clone(),
            field_type: match &field.enumerated_values {
                None => match field.bit_width {
                    1 => FieldType::Raw("bool".to_string()),
                    _ => FieldType::Raw("u32".to_string()),
                },
                Some(s) => FieldType::Enum(EnumeratedValues::from(s)),
            },
        }
    }
}

impl EnumeratedValues {
    fn from(enumerated_values: &input_svd::EnumeratedValues) -> EnumeratedValues {
        if enumerated_values.derived_from.is_some() {
            EnumeratedValues::Derived {
                derived_from: enumerated_values.derived_from.as_ref().unwrap().clone(),
            }
        } else {
            let enumerated_value = enumerated_values
                .enumerated_value
                .iter()
                .map(|ev| EnumeratedValue::from(&ev))
                .collect();
            EnumeratedValues::Content {
                name: enumerated_values.name.as_ref().unwrap().clone(),
                usage: enumerated_values.usage.clone(),
                enumerated_value,
            }
        }
    }
}

impl EnumeratedValue {
    fn from(enumerated_value: &input_svd::EnumeratedValue) -> EnumeratedValue {
        if enumerated_value.is_default.is_some() && enumerated_value.is_default.unwrap() {
            EnumeratedValue::Default {
                name: enumerated_value.name.clone(),
                description: enumerated_value.description.as_ref().unwrap().clone(),
            }
        } else {
            EnumeratedValue::Valued {
                name: enumerated_value.name.clone(),
                description: enumerated_value.description.as_ref().unwrap().clone(),
                value: enumerated_value.value.as_ref().unwrap().clone(),
            }
        }
    }
}

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