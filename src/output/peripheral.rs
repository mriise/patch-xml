use super::{AddressBlock, Register};
use crate::input::svd as input_svd;
use crate::output::{Interrupt, RegisterPropertiesGroup};
use itertools::Itertools;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimElementGroup {
    pub dim: Option<u32>,
    pub dim_increment: Option<u32>,
    pub dim_index: Option<u32>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<u32>,
}

impl DimElementGroup {
    fn from(dim_elem_group: &Option<input_svd::DimElementGroup>) -> Option<DimElementGroup> {
        match dim_elem_group {
            None => None,
            Some(deg) => Some(DimElementGroup {
                dim: match &deg.dim {
                    None => None,
                    Some(d) => Some(d.value),
                },
                dim_increment: match &deg.dim_increment {
                    None => None,
                    Some(di) => Some(di.value),
                },
                dim_index: match &deg.dim_index {
                    None => None,
                    Some(di) => Some(di.value),
                },
                dim_name: deg.dim_name.clone(),
                dim_array_index: match &deg.dim_array_index {
                    None => None,
                    Some(dai) => Some(dai.value),
                },
            }),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peripheral {
    pub derived_from: Option<String>,
    pub dim_element: Option<DimElementGroup>,
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
    pub registers: Vec<Register>,
}

impl Peripheral {
    pub fn from(peripheral: &input_svd::Peripheral) -> Peripheral {
        Peripheral {
            derived_from: peripheral.derived_from.clone(),
            dim_element: DimElementGroup::from(&peripheral.dim_element),
            name: peripheral.name.clone(),
            version: peripheral.version.clone(),
            description: peripheral.description.clone(),
            alternate_peripheral: peripheral.alternate_peripheral.clone(),
            group_name: peripheral.group_name.clone(),
            prepend_to_name: peripheral.prepend_to_name.clone(),
            append_to_name: peripheral.append_to_name.clone(),
            header_struct_name: peripheral.header_struct_name.clone(),
            disable_condition: peripheral.disable_condition.clone(),
            base_address: peripheral.base_address.clone(),
            register_properties: RegisterPropertiesGroup::from(&peripheral.register_properties),
            address_block: AddressBlock::from(&peripheral.address_block),
            interrupts: peripheral.interrupts.clone(),
            registers: peripheral
                .registers
                .as_ref()
                .unwrap()
                .register
                .iter()
                .map(|r| Register::from(&r))
                .sorted_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset))
                .collect(),
        }
        /*if peripheral.derived_from.is_some() {
            Peripheral::Derived {
                derived_from: peripheral.derived_from.as_ref().unwrap().clone(),
                name: peripheral.name.clone(),
                base_address: peripheral.base_address.clone(),
            }
        } else {
            Peripheral::Content {
                name: peripheral.name.clone(),
                base_address: peripheral.base_address.clone(),
                address_block: AddressBlock::from(&peripheral.address_block),
                version: peripheral.version.clone(),
                description: peripheral.description.as_ref().unwrap().clone(),
                group_name: peripheral.group_name.clone(),
                prepend_to_name: peripheral.prepend_to_name.clone(),
                append_to_name: peripheral.append_to_name.clone(),
                disable_condition: peripheral.disable_condition.clone(),
                registers: peripheral
                    .registers
                    .as_ref()
                    .unwrap()
                    .register
                    .iter()
                    .map(|r| Register::from(&r))
                    .sorted_by(|r1, r2| r1.address_offset.cmp(&r2.address_offset))
                    .collect(),
                atomic_registers: vec![],
            }
        }*/
    }
}
