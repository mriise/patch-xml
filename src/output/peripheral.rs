use super::{AddressBlock, Cluster, Interrupt, Register};
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Peripheral {
    pub derived_from: Option<String>,
    pub dim: Option<SvdConstant>,
    pub dim_increment: Option<SvdConstant>,
    pub dim_index: Option<SvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<DimArrayIndex>,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub alternate_peripheral: Option<String>,
    pub group_name: Option<String>,
    pub prepend_to_name: Option<String>,
    pub append_to_name: Option<String>,
    pub header_struct_name: Option<String>,
    pub disable_condition: Option<String>,
    pub base_address: SvdConstant,
    pub size: Option<SvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<SvdConstant>,
    pub reset_mask: Option<SvdConstant>,
    pub address_block: Option<AddressBlock>,
    pub interrupt: Option<Vec<Interrupt>>,
    pub registers: Option<Registers>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Registers {
    pub cluster: Option<Vec<Cluster>>,
    pub register: Vec<Register>,
}
