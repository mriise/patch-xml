use super::{AccessType, Field, ModifiedWriteValues, ReadAction, WriteConstraint};
use serde::Serialize;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub name: String,
    pub size: u32,
    pub reset_value: u32,
    //Supported by yaml-config
    pub display_name: String,
    pub description: String,
    pub access: Option<AccessType>,

    pub alternate_group: Option<String>,
    pub address_offset: u32,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub read_fields: Vec<Field>,
    pub write_fields: Vec<Field>,
    pub read_write_fields: Vec<Field>,
}
