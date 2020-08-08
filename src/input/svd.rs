use crate::output::{
    AccessType, EndianType, EnumeratedValuesUsage, Interrupt, ModifiedWriteValues, Protection,
    ReadAction, SvdConstant, WriteConstraint,
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Svd {
    pub device: Device,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPropertiesGroup {
    pub size: Option<NestedSvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<NestedSvdConstant>,
    pub reset_mask: Option<NestedSvdConstant>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    #[serde(with = "SvdConstant")]
    pub base: u32,
    #[serde(with = "SvdConstant")]
    pub limit: u32,
    pub access: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub name: String,
    pub series: Option<String>,
    pub version: String,
    pub description: String,
    pub license_text: Option<String>,
    pub cpu: Cpu,
    pub header_system_filename: Option<String>,
    pub header_definition_prefix: Option<String>,
    pub address_unit_bits: u32,
    pub width: u32,
    #[serde(flatten)]
    pub register_properties: RegisterPropertiesGroup,
    pub peripherals: Peripherals,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    pub name: String,
    pub revision: String,
    pub endian: EndianType,
    pub mpu_present: bool,
    pub fpu_present: bool,
    pub fpu_d_p: Option<bool>,
    pub dsp_present: Option<bool>,
    pub icache_present: Option<bool>,
    pub dcache_present: Option<bool>,
    pub itcm_present: Option<bool>,
    pub dtcm_present: Option<bool>,
    pub vtor_present: Option<bool>,
    #[serde(with = "SvdConstant")]
    pub nvic_prio_bits: u32,
    pub vendor_systick_config: bool,
    pub device_num_interrupts: Option<NestedSvdConstant>,
    pub sau_num_regions: Option<NestedSvdConstant>,
    pub sau_regions_config: Option<SauRegionsConfigType>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Peripherals {
    pub peripheral: Vec<Peripheral>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
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
    #[serde(with = "SvdConstant")]
    pub base_address: u32,
    pub register_properties: RegisterPropertiesGroup,
    pub address_block: Option<AddressBlock>,
    pub interrupts: Vec<Interrupt>,
    pub registers: Option<Registers>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimElementGroup {
    pub dim: Option<NestedSvdConstant>,
    pub dim_increment: Option<NestedSvdConstant>,
    pub dim_index: Option<NestedSvdConstant>,
    pub dim_name: Option<String>,
    pub dim_array_index: Option<NestedSvdConstant>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    #[serde(with = "SvdConstant")]
    pub offset: u32,
    #[serde(with = "SvdConstant")]
    pub size: u32,
    pub usage: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalStructItem {
    pub name: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Registers {
    pub register: Vec<Register>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    //Not yet supported by yaml-config
    pub name: String,
    #[serde(with = "SvdConstant")]
    pub size: u32,
    #[serde(with = "SvdConstant")]
    pub reset_value: u32,
    //Supported by yaml-config
    pub display_name: String,
    pub description: String,
    pub access: Option<AccessType>,
    pub derived_from: Option<String>,

    pub alternate_group: Option<String>,
    #[serde(with = "SvdConstant")]
    pub address_offset: u32,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub fields: Fields,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    pub field: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    //Not yet supported by yaml-config
    pub name: String,
    pub bit_offset: u32,
    pub bit_width: u32,
    //Supported by yaml-config
    pub derived_from: Option<String>,
    pub description: Option<String>,
    pub access: Option<AccessType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub write_constraint: Option<WriteConstraint>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Option<EnumeratedValues>,
    pub enumerated_values2: Option<EnumeratedValues>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValues {
    pub name: Option<String>,
    pub derived_from: Option<String>,
    pub usage: Option<EnumeratedValuesUsage>,
    pub enumerated_value: Vec<EnumeratedValue>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    pub name: String,
    pub description: Option<String>,
    pub value: Option<u32>,
    pub is_default: Option<bool>,
}

impl Svd {
    pub fn read(svd_path: String) -> Svd {
        Svd {
            device: serde_xml_rs::from_str(
                fs::read_to_string(svd_path)
                    .expect("Error while reading svd file")
                    .as_str(),
            )
            .expect("Error while parsing SVD file"),
        }
    }
    pub fn write(&self, path: &String) {
        let write_result = match serde_xml_rs::to_string(self) {
            Ok(svd_string) => fs::write(&path, svd_string.as_bytes()),
            Err(e) => panic!("Could not serialize SVD struct: {}", e),
        };
        if write_result.is_err() {
            panic!(
                "Error while writing SVD to disk: {}",
                write_result.unwrap_err()
            );
        }
    }
}

impl EnumeratedValues {
    pub fn new() -> EnumeratedValues {
        EnumeratedValues {
            name: None,
            derived_from: None,
            usage: None,
            enumerated_value: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NestedSvdConstant {
    #[serde(rename = "$value", with = "SvdConstant")]
    pub value: u32,
}
