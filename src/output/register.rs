use super::{DataType, Field, ModifiedWriteValues, ReadAction, WriteConstraint};
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    pub derived_from: Option<String>,
    pub dim: Option<SvdConstant>,
    pub dim_increment: Option<SvdConstant>,
    pub dim_index: Option<SvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<DimArrayIndex>,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub alternate_group: Option<String>,
    pub alternate_register: Option<String>,
    pub address_offset: SvdConstant,
    pub size: Option<SvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<SvdConstant>,
    pub reset_mask: Option<SvdConstant>,
    pub data_type: Option<DataType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub fields: Option<Fields>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub field: Vec<Field>,
}
