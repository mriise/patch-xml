use super::{AccessType, ModifiedWriteValues, WriteConstraint, ReadAction, EnumeratedValuesUsage};
use serde::{Serialize};
use crate::input::svd as input_svd;

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
        value: u32,
    },
    Default {
        name: String,
        description: String,
    },
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
