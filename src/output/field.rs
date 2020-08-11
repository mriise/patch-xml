use super::{AccessType, EnumeratedValuesUsage, ModifiedWriteValues, ReadAction, WriteConstraint};
use serde::Serialize;

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
