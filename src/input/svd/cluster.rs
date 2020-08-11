use crate::output::{Cluster as OutputCluster, SvdConstant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Clusters {
    pub clusters: Vec<Cluster>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    pub derived_from: Option<String>,
    pub dim_element_group: super::DimElementGroup,
    pub name: String,
    pub description: Option<String>,
    pub alternate_cluster: Option<String>,
    pub header_struct_name: Option<String>,
    #[serde(with = "SvdConstant")]
    pub address_offset: u32,
    pub register_properties_group: super::RegisterPropertiesGroup,
    pub registers: Vec<super::Register>,
    pub clusters: Vec<Cluster>,
}

impl Clusters {
    pub fn to_output(&self) -> Vec<OutputCluster> {
        self.clusters.iter().map(Cluster::to_output).collect()
    }
}

impl Cluster {
    pub fn to_output(&self) -> OutputCluster {
        OutputCluster {
            derived_from: self.derived_from.clone(),
            dim_element_group: self.dim_element_group.to_output(),
            name: self.name.clone(),
            description: self.description.clone(),
            alternate_cluster: self.alternate_cluster.clone(),
            header_struct_name: self.header_struct_name.clone(),
            address_offset: self.address_offset.clone(),
            register_properties_group: self.register_properties_group.to_output(),
            registers: self
                .registers
                .iter()
                .map(super::Register::to_output)
                .collect(),
            clusters: self.clusters.iter().map(Cluster::to_output).collect(),
        }
    }
}
