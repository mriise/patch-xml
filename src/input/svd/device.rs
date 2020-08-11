use crate::output::{
    Cpu as OutputCpu, Device as OutputDevice, EndianType,
    RegisterPropertiesGroup as OutputRegisterPropertiesGroup, SvdConstant,
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

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
    pub register_properties: super::RegisterPropertiesGroup,
    pub peripherals: super::Peripherals,
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
    pub device_num_interrupts: Option<super::NestedSvdConstant>,
    pub sau_num_regions: Option<super::NestedSvdConstant>,
    pub sau_regions_config: Option<super::SauRegionsConfigType>,
}

impl Device {
    pub(crate) fn to_output(&self) -> OutputDevice {
        OutputDevice {
            vendor: self.vendor.clone(),
            vendor_id: self.vendor_id.clone(),
            name: self.name.clone(),
            series: self.series.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            license_text: self.license_text.clone(),
            cpu: OutputCpu {
                name: self.cpu.name.clone(),
                revision: self.cpu.revision.clone(),
                endian: self.cpu.endian.clone(),
                mpu_present: self.cpu.mpu_present.clone(),
                fpu_present: self.cpu.fpu_present.clone(),
                fpu_d_p: self.cpu.fpu_d_p.clone(),
                dsp_present: self.cpu.dsp_present.clone(),
                icache_present: self.cpu.icache_present.clone(),
                dcache_present: self.cpu.dcache_present.clone(),
                itcm_present: self.cpu.itcm_present.clone(),
                dtcm_present: self.cpu.dtcm_present.clone(),
                vtor_present: self.cpu.vtor_present.clone(),
                nvic_prio_bits: self.cpu.nvic_prio_bits.clone(),
                vendor_systick_config: self.cpu.vendor_systick_config.clone(),
                device_num_interrupts: match &self.cpu.device_num_interrupts {
                    None => None,
                    Some(dni) => Some(dni.value.clone()),
                },
                sau_num_regions: match &self.cpu.sau_num_regions {
                    None => None,
                    Some(snr) => Some(snr.value.clone()),
                },
                sau_regions_config: match &self.cpu.sau_regions_config {
                    None => None,
                    Some(src) => Some(src.to_output()),
                },
            },
            header_system_filename: self.header_system_filename.clone(),
            header_definition_prefix: self.header_definition_prefix.clone(),
            address_unit_bits: self.address_unit_bits.clone(),
            width: self.width.clone(),
            register_properties: OutputRegisterPropertiesGroup {
                size: self
                    .register_properties
                    .size
                    .as_ref()
                    .map(|s| s.value.clone()),
                access: self.register_properties.access.clone(),
                protection: self.register_properties.protection.clone(),
                reset_value: self
                    .register_properties
                    .reset_value
                    .as_ref()
                    .map(|s| s.value.clone()),
                reset_mask: self
                    .register_properties
                    .reset_mask
                    .as_ref()
                    .map(|s| s.value.clone()),
            },
            peripherals: self
                .peripherals
                .peripheral
                .iter()
                .sorted_by(|p1, p2| p1.base_address.cmp(&p2.base_address))
                .map(|p| p.to_output())
                .collect(),
        }
    }
}
