use super::common::*;
use super::{svd, svd::Svd};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    pub svd: String,
    pub device: Device,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Device {
    pub peripherals: HashMap<RegExStruct, Peripheral>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Peripheral {
    pub patch_type: Option<PatchType>,
    pub derived_from: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub group_name: Option<String>,
    pub prepend_to_name: Option<String>,
    pub append_to_name: Option<String>,
    pub disable_condition: Option<String>,
    #[serde(flatten)]
    pub registers: HashMap<RegExStruct, Register>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Register {
    pub patch_type: Option<PatchType>,
    pub derived_from: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub access: Option<AccessType>,
    pub alternate_group: Option<String>,
    pub address_offset: Option<u32>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    #[serde(flatten)]
    pub fields: HashMap<RegExStruct, Field>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub patch_type: Option<PatchType>,
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Option<EnumeratedValues>,
    pub enumerated_values2: Option<EnumeratedValues>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct EnumeratedValues {
    pub _patch_type: Option<PatchType>,
    pub name: Option<String>,
    pub derived_from: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: HashMap<RegExStruct, EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum EnumeratedValue {
    Value(u32),
    ValueDescription(u32, String),
    DefaultDescription(String),
    Default,
}

impl Config {
    pub fn read(config_filename: &str) -> Config {
        serde_yaml::from_str(
            fs::read_to_string(config_filename)
                .expect(format!("Error while reading svd patch file {}", config_filename).as_str())
                .as_str(),
        )
        .expect("Error while parsing configuration file")
    }
    pub fn merge_into(&self, svd: &mut Svd) {
        for (peripheral_name, peripherals) in &self.device.peripherals {
            for svd_peripheral in &mut svd.device.peripherals.peripheral {
                if peripheral_name.regex.is_match(&svd_peripheral.name) {
                    peripherals.merge_into(svd_peripheral);
                }
            }
        }
    }
}

impl EnumeratedValue {
    pub fn merge_into(&self, svd_enum_value: &mut svd::EnumeratedValue) {
        //Create
        let (value, description, is_default) = match self {
            EnumeratedValue::Value(v) => (Some(v.clone()), None, false),
            EnumeratedValue::ValueDescription(v, d) => (Some(v.clone()), Some(d.clone()), false),
            EnumeratedValue::DefaultDescription(d) => (None, Some(d.clone()), true),
            EnumeratedValue::Default => (None, None, true),
        };

        if value.is_some() {
            svd_enum_value.value = value;
        }
        if description.is_some() {
            svd_enum_value.description = description;
        }
        svd_enum_value.is_default = Some(is_default);
    }
}

macro_rules! merge_option_property {
    ($source:ident, $target:ident, $field:ident) => {
        if $source.$field.is_some() {
            $target.$field = $source.$field.clone();
        }
    };
}

macro_rules! merge_property {
    ($source:ident, $target:ident, $field:ident) => {
        if $source.$field.is_some() {
            $target.$field = $source.$field.as_ref().unwrap().clone();
        }
    };
}

impl Peripheral {
    pub fn merge_into(&self, svd_peripheral: &mut svd::Peripheral) {
        merge_option_property! {self, svd_peripheral, derived_from}
        merge_option_property! {self, svd_peripheral, version}
        merge_option_property! {self, svd_peripheral, description}
        merge_option_property! {self, svd_peripheral, group_name}
        merge_option_property! {self, svd_peripheral, prepend_to_name}
        merge_option_property! {self, svd_peripheral, append_to_name}
        merge_option_property! {self, svd_peripheral, disable_condition}
        for (register_name, register) in &self.registers {
            if svd_peripheral.registers.is_some() {
                for svd_register in &mut svd_peripheral.registers.as_mut().unwrap().register {
                    if register_name.regex.is_match(&svd_register.name) {
                        register.merge_into(svd_register);
                    }
                }
            }
        }
    }
}

impl Register {
    pub fn merge_into(&self, svd_register: &mut svd::Register) {
        merge_property! {self, svd_register, display_name}
        merge_property! {self, svd_register, description}
        merge_option_property! {self, svd_register, access}
        merge_option_property! {self, svd_register, derived_from}
        merge_option_property! {self, svd_register, alternate_group}
        merge_property! {self, svd_register, address_offset}
        merge_option_property! {self, svd_register, modified_write_values}
        merge_option_property! {self, svd_register, write_constraint}
        merge_option_property! {self, svd_register, read_action}
        for (field_name, field) in &self.fields {
            for svd_field in &mut svd_register.fields.field {
                if field_name.regex.is_match(&svd_field.name) {
                    field.merge_into(svd_field);
                }
            }
        }
    }
}

impl Field {
    pub fn merge_into(&self, svd_field: &mut svd::Field) {
        merge_option_property! {self, svd_field, derived_from}
        merge_option_property! {self, svd_field, description}
        merge_option_property! {self, svd_field, access}
        merge_option_property! {self, svd_field, modified_write_values}
        merge_option_property! {self, svd_field, write_constraint}
        merge_option_property! {self, svd_field, read_action}
        if self.enumerated_values.is_some() {
            if svd_field.enumerated_values.is_none() {
                svd_field.enumerated_values = Some(svd::EnumeratedValues::new());
            }
            let svd_enumerated_values = svd_field.enumerated_values.as_mut().unwrap();
            self.enumerated_values
                .as_ref()
                .unwrap()
                .merge_into(svd_enumerated_values);
        }
        if self.enumerated_values2.is_some() {
            if svd_field.enumerated_values2.is_none() {
                svd_field.enumerated_values2 = Some(svd::EnumeratedValues::new());
            }
            let svd_enumerated_values2 = svd_field.enumerated_values2.as_mut().unwrap();
            self.enumerated_values2
                .as_ref()
                .unwrap()
                .merge_into(svd_enumerated_values2);
        }
    }
}

impl EnumeratedValues {
    pub fn merge_into(&self, svd_enum_values: &mut svd::EnumeratedValues) {
        merge_option_property!(self, svd_enum_values, name);
        merge_option_property!(self, svd_enum_values, derived_from);
        merge_option_property!(self, svd_enum_values, usage);
        for (enum_value_name, enum_value) in &self.enumerated_value {
            for svd_enum_value in &mut svd_enum_values.enumerated_value {
                if enum_value_name.regex.is_match(&svd_enum_value.name) {
                    enum_value.merge_into(svd_enum_value);
                }
            }
        }
    }
}
