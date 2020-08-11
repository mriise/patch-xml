use crate::output::{Interrupt, Peripheral as OutputPeripheral, SvdConstant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Peripherals {
    pub peripheral: Vec<Peripheral>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peripheral {
    pub derived_from: Option<String>,
    pub dim_element: super::DimElementGroup,
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub alternate_peripheral: Option<String>,
    pub group_name: Option<String>,
    pub prepend_to_name: Option<String>,
    pub append_to_name: Option<String>,
    pub header_struct_name: Option<String>,
    pub disable_condition: Option<String>,
    #[serde(with = "SvdConstant")]
    pub base_address: u32,
    pub register_properties: super::RegisterPropertiesGroup,
    pub address_block: Option<super::AddressBlock>,
    pub interrupts: Vec<Interrupt>,
    pub clusters: Option<super::Clusters>,
    pub registers: Option<super::Registers>,
}

impl Peripheral {
    pub fn to_output(&self) -> OutputPeripheral {
        OutputPeripheral {
            derived_from: self.derived_from.clone(),
            dim_element: self.dim_element.to_output(),
            name: self.name.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            alternate_peripheral: self.alternate_peripheral.clone(),
            group_name: self.group_name.clone(),
            prepend_to_name: self.prepend_to_name.clone(),
            append_to_name: self.append_to_name.clone(),
            header_struct_name: self.header_struct_name.clone(),
            disable_condition: self.disable_condition.clone(),
            base_address: self.base_address.clone(),
            register_properties: self.register_properties.to_output(),
            address_block: match &self.address_block {
                None => None,
                Some(ab) => Some(ab.to_output()),
            },
            interrupts: self.interrupts.clone(),
            clusters: match &self.clusters {
                None => Vec::new(),
                Some(clusters) => clusters.to_output(),
            },
            registers: match &self.registers {
                None => Vec::new(),
                Some(regs) => regs.to_output(),
            },
        }
    }
}
