use super::{
    AccessType, EnumeratedValue, EnumeratedValuesUsage, ModifiedWriteValues, ReadAction,
    WriteConstraint,
};
use crate::output::{DimArrayIndex, SvdConstant};
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub derived_from: Option<String>,
    pub dim: Option<SvdConstant>,
    pub dim_increment: Option<SvdConstant>,
    pub dim_index: Option<SvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<DimArrayIndex>,
    pub name: String,
    pub description: Option<String>,
    pub bit_offset: Option<SvdConstant>,
    pub bit_width: Option<SvdConstant>,
    pub lsb: Option<SvdConstant>,
    pub msb: Option<SvdConstant>,
    pub bit_range: Option<SvdConstant>,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Option<EnumAccessType>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValues {
    pub derived_from: Option<String>,
    pub name: Option<String>,
    pub header_enum_name: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum EnumAccessType {
    ReadAndWrite(EnumeratedValues),
    ReadWrite {
        read: EnumeratedValues,
        write: EnumeratedValues,
    },
    Read(EnumeratedValues),
    Write(EnumeratedValues),
}
