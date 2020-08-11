use super::{EndianType, Peripheral, RegisterPropertiesGroup};
use serde::Serialize;

#[derive(Serialize, Clone)]
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
    pub register_properties: RegisterPropertiesGroup,
    pub peripherals: Vec<Peripheral>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
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
    pub nvic_prio_bits: u32,
    pub vendor_systick_config: bool,
    pub device_num_interrupts: Option<u32>,
    pub sau_num_regions: Option<u32>,
    pub sau_regions_config: Option<SauRegionsConfigType>,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub base: u32,
    pub limit: u32,
    pub access: String,
}
