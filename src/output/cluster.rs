use super::{DimElementGroup, Register, RegisterPropertiesGroup};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub derived_from: Option<String>,
    pub dim_element_group: DimElementGroup,
    pub name: String,
    pub description: Option<String>,
    pub alternate_cluster: Option<String>,
    pub header_struct_name: Option<String>,
    pub address_offset: u32,
    pub register_properties_group: RegisterPropertiesGroup,
    pub registers: Vec<Register>,
    pub clusters: Vec<Cluster>,
}
