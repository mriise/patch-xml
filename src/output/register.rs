use super::{
    DataType, DimElementGroup, Field, ModifiedWriteValues, ReadAction, RegisterPropertiesGroup,
    WriteConstraint,
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub derived_from: Option<String>,
    pub dim_element: DimElementGroup,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    pub address_offset: u32,
    pub register_properties: RegisterPropertiesGroup,
    pub data_type: Option<DataType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub fields: Vec<Field>,
}
