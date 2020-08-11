use crate::output::{
    AccessType, EnumeratedValue as OutputEnumeratedValue,
    EnumeratedValues as OutputEnumeratedValues, EnumeratedValuesUsage, Field as OutputField,
    FieldType, ModifiedWriteValues, ReadAction, WriteConstraint,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub field: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub derived_from: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    //Not yet supported by yaml-config
    pub name: String,
    pub bit_offset: u32,
    pub bit_width: u32,
    //Supported by yaml-config
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Option<EnumeratedValues>,
    pub enumerated_values2: Option<EnumeratedValues>,
}

impl EnumeratedValues {
    pub fn new() -> EnumeratedValues {
        EnumeratedValues {
            name: None,
            derived_from: None,
            usage: None,
            enumerated_value: Vec::new(),
        }
    }
}

impl Field {
    pub fn to_output(&self) -> OutputField {
        OutputField {
            name: self.name.clone(),
            mask: ((((1 as usize) << self.bit_width as usize) - 1) << self.bit_offset) as u32,
            description: self.description.clone(),
            access: self.access.clone(),
            modified_write_values: self.modified_write_values.clone(),
            write_constraint: self.write_constraint.clone(),
            read_action: self.read_action.clone(),
            field_type: match &self.enumerated_values {
                None => match self.bit_width {
                    1 => FieldType::Raw("bool".to_string()),
                    _ => FieldType::Raw("u32".to_string()),
                },
                Some(s) => FieldType::Enum(s.to_output()),
            },
        }
    }
}

impl EnumeratedValues {
    fn to_output(&self) -> OutputEnumeratedValues {
        if self.derived_from.is_some() {
            OutputEnumeratedValues::Derived {
                derived_from: self.derived_from.as_ref().unwrap().clone(),
            }
        } else {
            let enumerated_value = self
                .enumerated_value
                .iter()
                .map(|ev| ev.to_output())
                .collect();
            OutputEnumeratedValues::Content {
                name: self.name.as_ref().unwrap().clone(),
                usage: self.usage.clone(),
                enumerated_value,
            }
        }
    }
}

impl EnumeratedValue {
    fn to_output(&self) -> OutputEnumeratedValue {
        if self.is_default.is_some() && self.is_default.unwrap() {
            OutputEnumeratedValue::Default {
                name: self.name.clone(),
                description: self.description.as_ref().unwrap().clone(),
            }
        } else {
            OutputEnumeratedValue::Valued {
                name: self.name.clone(),
                description: self.description.as_ref().unwrap().clone(),
                value: self.value.as_ref().unwrap().clone(),
            }
        }
    }
}
