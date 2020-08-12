use super::{
    AccessType, DimElementGroup, EnumeratedValue, EnumeratedValuesUsage, ModifiedWriteValues,
    ReadAction, WriteConstraint,
};
use serde::Serialize;

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub derived_from: Option<String>,
    pub dim_element: DimElementGroup,
    pub name: String,
    pub description: Option<String>,
    pub mask: u32,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: EnumAccessType,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub struct EnumeratedValues {
    pub derived_from: Option<String>,
    pub name: Option<String>,
    pub header_enum_name: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
pub enum EnumAccessType {
    ReadAndWrite(EnumeratedValues),
    ReadWrite {
        read: EnumeratedValues,
        write: EnumeratedValues,
    },
    Read(EnumeratedValues),
    Write(EnumeratedValues),
    None,
}
