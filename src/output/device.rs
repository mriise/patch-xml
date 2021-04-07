use super::{EndianType, Peripheral};
use crate::output::{AccessType, Protection, SvdConstant};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub schema_version: String,
    pub vendor: Option<String>,
    pub vendor_id: Option<String>,
    pub name: String,
    pub series: Option<String>,
    pub version: String,
    pub description: String,
    pub license_text: Option<String>,
    pub cpu: Cpu,
    pub header_system_filename: Option<String>,
    pub header_definitions_prefix: Option<String>,
    pub address_unit_bits: SvdConstant,
    pub width: SvdConstant,
    pub size: Option<SvdConstant>,
    pub access: Option<AccessType>,
    pub protection: Option<Protection>,
    pub reset_value: Option<SvdConstant>,
    pub reset_mask: Option<SvdConstant>,
    pub peripherals: Peripherals,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
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
    pub nvic_prio_bits: SvdConstant,
    pub vendor_systick_config: bool,
    pub device_num_interrupts: Option<SvdConstant>,
    pub sau_num_regions: Option<SvdConstant>,
    pub sau_regions_config: Option<SauRegionsConfigType>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    pub enabled: Option<bool>,
    pub protection_when_disabled: Option<Protection>,
    pub regions: Option<SauRegionsType>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsType {
    pub region: Vec<SauRegionType>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionType {
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub base: SvdConstant,
    pub limit: SvdConstant,
    pub access: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peripherals {
    pub peripheral: Vec<Peripheral>,
}
