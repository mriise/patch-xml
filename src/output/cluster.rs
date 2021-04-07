use super::Register;
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub derived_from: Option<String>,
    pub dim: Option<SvdConstant>,
    pub dim_increment: Option<SvdConstant>,
    pub dim_index: Option<SvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<DimArrayIndex>,
    pub name: String,
    pub description: Option<String>,
    pub alternate_cluster: Option<String>,
    pub header_struct_name: Option<String>,
    pub address_offset: SvdConstant,
    pub size: Option<SvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<SvdConstant>,
    pub reset_mask: Option<SvdConstant>,
    pub register: Option<Vec<Register>>,
    pub cluster: Option<Vec<Cluster>>,
}
