use super::{EndianType, Peripheral, RegisterPropertiesGroup};
use crate::input::svd as input_svd;
use itertools::Itertools;
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

impl SauRegionsConfigType {
    pub fn from(
        address_block: &Option<crate::input::svd::SauRegionsConfigType>,
    ) -> Option<SauRegionsConfigType> {
        match address_block {
            None => None,
            Some(src) => Some(SauRegionsConfigType {
                enabled: src.enabled.clone(),
                name: src.name.clone(),
                base: src.base.clone(),
                limit: src.limit.clone(),
                access: src.access.clone(),
            }),
        }
    }
}

impl Device {
    pub fn from(input_svd: &input_svd::Svd) -> Device {
        Device {
            vendor: input_svd.device.vendor.clone(),
            vendor_id: input_svd.device.vendor_id.clone(),
            name: input_svd.device.name.clone(),
            series: input_svd.device.series.clone(),
            version: input_svd.device.version.clone(),
            description: input_svd.device.description.clone(),
            license_text: input_svd.device.license_text.clone(),
            cpu: Cpu {
                name: input_svd.device.cpu.name.clone(),
                revision: input_svd.device.cpu.revision.clone(),
                endian: input_svd.device.cpu.endian.clone(),
                mpu_present: input_svd.device.cpu.mpu_present.clone(),
                fpu_present: input_svd.device.cpu.fpu_present.clone(),
                fpu_d_p: input_svd.device.cpu.fpu_d_p.clone(),
                dsp_present: input_svd.device.cpu.dsp_present.clone(),
                icache_present: input_svd.device.cpu.icache_present.clone(),
                dcache_present: input_svd.device.cpu.dcache_present.clone(),
                itcm_present: input_svd.device.cpu.itcm_present.clone(),
                dtcm_present: input_svd.device.cpu.dtcm_present.clone(),
                vtor_present: input_svd.device.cpu.vtor_present.clone(),
                nvic_prio_bits: input_svd.device.cpu.nvic_prio_bits.clone(),
                vendor_systick_config: input_svd.device.cpu.vendor_systick_config.clone(),
                device_num_interrupts: match &input_svd.device.cpu.device_num_interrupts {
                    None => None,
                    Some(dni) => Some(dni.value.clone()),
                },
                sau_num_regions: match &input_svd.device.cpu.sau_num_regions {
                    None => None,
                    Some(snr) => Some(snr.value.clone()),
                },
                sau_regions_config: SauRegionsConfigType::from(
                    &input_svd.device.cpu.sau_regions_config,
                ),
            },
            header_system_filename: input_svd.device.header_system_filename.clone(),
            header_definition_prefix: input_svd.device.header_definition_prefix.clone(),
            address_unit_bits: input_svd.device.address_unit_bits.clone(),
            width: input_svd.device.width.clone(),
            register_properties: RegisterPropertiesGroup {
                size: input_svd
                    .device
                    .register_properties
                    .size
                    .as_ref()
                    .map(|s| s.value.clone()),
                access: input_svd.device.register_properties.access.clone(),
                protection: input_svd.device.register_properties.protection.clone(),
                reset_value: input_svd
                    .device
                    .register_properties
                    .reset_value
                    .as_ref()
                    .map(|s| s.value.clone()),
                reset_mask: input_svd
                    .device
                    .register_properties
                    .reset_mask
                    .as_ref()
                    .map(|s| s.value.clone()),
            },
            peripherals: input_svd
                .device
                .peripherals
                .peripheral
                .iter()
                .sorted_by(|p1, p2| p1.base_address.cmp(&p2.base_address))
                .map(|p| Peripheral::from(&p))
                .collect(),
        }
    }
}
