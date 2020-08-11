use super::{AddressBlock, Cluster, DimElementGroup, Interrupt, Register, RegisterPropertiesGroup};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peripheral {
    pub derived_from: Option<String>,
    pub dim_element: DimElementGroup,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub alternate_peripheral: Option<String>,
    pub group_name: Option<String>,
    pub prepend_to_name: Option<String>,
    pub append_to_name: Option<String>,
    pub header_struct_name: Option<String>,
    pub disable_condition: Option<String>,
    pub base_address: u32,
    pub register_properties: RegisterPropertiesGroup,
    pub address_block: Option<AddressBlock>,
    pub interrupts: Vec<Interrupt>,
    pub clusters: Vec<Cluster>,
    pub registers: Vec<Register>,
}
