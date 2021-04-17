use super::{EndianType, Peripheral};
use crate::output::{AccessType, CpuNameType, Protection, SvdConstant};
use serde::Deserialize;

/// The element <device> provides the outermost frame of the description.
///
///  - Only one <device> section is allowed per file. All other elements are described within this scope.
///  - A <device> contains one or more peripherals, but one <cpu> description.
///  - Optional elements such as <size>, <access>, or <resetValue> defined on this level represent default values for registers and can be refined at lower levels.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Specify the compliant CMSIS-SVD schema version (for example, 1.1).
    pub schema_version: String,
    /// Specify the vendor of the device using the full name.
    pub vendor: Option<String>,
    /// Specify the vendor abbreviation without spaces or special characters.
    ///
    /// This information is used to define the directory.
    pub vendor_id: Option<String>,
    /// The string identifies the device or device series.
    ///
    /// Device names are required to be unique.
    pub name: String,
    /// Specify the name of the device series.
    pub series: Option<String>,
    /// Define the version of the SVD file.
    ///
    /// Silicon vendors maintain the description throughout the life-cycle of the device and ensure that all updated and released copies have a unique version string. Higher numbers indicate a more recent version.
    pub version: String,
    /// Describe the main features of the device (for example CPU, clock frequency, peripheral overview).
    pub description: String,
    /// The text will be copied into the header section of the generated device header file and shall contain the legal disclaimer.
    ///
    /// New lines can be inserted by using \n. This section is mandatory if the SVD file is used for generating the device header file.
    pub license_text: Option<String>,
    /// Describe the processor included in the device.
    pub cpu: Cpu,
    /// Specify the file name (without extension) of the device-specific system include file (system_<device>.h; See CMSIS-Core description).
    ///
    /// The header file generator customizes the include statement referencing the CMSIS system file within the CMSIS device header file. By default, the filename is system_device-name.h. In cases where a device series shares a single system header file, the name of the series shall be used instead of the individual device name.
    pub header_system_filename: Option<String>,
    /// This string is prepended to all type definition names generated in the CMSIS-Core device header file.
    ///
    /// This is used if the vendor's software requires vendor-specific types in order to avoid name clashes with other definied types.
    pub header_definitions_prefix: Option<String>,
    /// Define the number of data bits uniquely selected by each address.
    ///
    /// The value for Cortex-M-based devices is 8 (byte-addressable).
    pub address_unit_bits: SvdConstant,
    /// Define the number of data bit-width of the maximum single data transfer supported by the bus infrastructure.
    ///
    /// This information is relevant for debuggers when accessing registers, because it might be required to issue multiple accesses for resources of a bigger size. The expected value for Cortex-M-based devices is 32.
    pub width: SvdConstant,
    /// Default bit-width of any register contained in the device.
    pub size: Option<SvdConstant>,
    /// Default access rights for all registers.
    pub access: Option<AccessType>,
    /// Default access protection for all registers.
    pub protection: Option<Protection>,
    /// Default value for all registers at RESET.
    pub reset_value: Option<SvdConstant>,
    /// Define which register bits have a defined reset value.
    pub reset_mask: Option<SvdConstant>,
    /// Group to define peripherals.
    pub peripherals: Peripherals,
}

/// The CPU section describes the processor included in the microcontroller device.
///
/// This section is mandatory if the SVD file is used to generate the device header file.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cpu {
    /// CPU name
    pub name: CpuNameType,
    /// Define the HW revision of the processor.
    ///
    /// The version format is rNpM (N,M = \[0 - 99\]).
    pub revision: String,
    /// Define the endianness of the processor
    pub endian: EndianType,
    /// Indicate whether the processor is equipped with a memory protection unit (MPU).
    pub mpu_present: bool,
    /// Indicate whether the processor is equipped with a hardware floating point unit (FPU).
    ///
    /// Cortex-M4, Cortex-M7, Cortex-M33 and Cortex-M35P are the only available Cortex-M processor with an optional FPU.
    pub fpu_present: bool,
    /// Indicate whether the processor is equipped with a double precision floating point unit.
    ///
    /// This element is valid only when <fpuPresent> is set to true. Currently, only Cortex-M7 processors can have a double precision floating point unit.
    pub fpu_d_p: Option<bool>,
    /// Indicate whether the processor is equipped with a double precision floating point unit.
    ///
    /// This element is valid only when <fpuPresent> is set to true. Currently, only Cortex-M7 processors can have a double precision floating point unit.
    pub dsp_present: Option<bool>,
    /// Indicate whether the processor has an instruction cache.
    ///
    /// Note: only for Cortex-M7-based devices.
    pub icache_present: Option<bool>,
    /// Indicate whether the processor has a data cache. Note: only for Cortex-M7-based devices.
    pub dcache_present: Option<bool>,
    /// Indicate whether the processor has an instruction tightly coupled memory.
    ///
    /// Note: only an option for Cortex-M7-based devices.
    pub itcm_present: Option<bool>,
    /// Indicate whether the processor has a data tightly coupled memory.
    ///
    /// Note: only for Cortex-M7-based devices.
    pub dtcm_present: Option<bool>,
    /// Indicate whether the Vector Table Offset Register (VTOR) is implemented in Cortex-M0+ based devices.
    ///
    /// If not specified, then VTOR is assumed to be present.
    pub vtor_present: Option<bool>,
    /// Define the number of bits available in the Nested Vectored Interrupt Controller (NVIC) for configuring priority.
    pub nvic_prio_bits: SvdConstant,
    /// Indicate whether the processor implements a vendor-specific System Tick Timer.
    ///
    /// If false, then the Arm-defined System Tick Timer is available. If true, then a vendor-specific System Tick Timer must be implemented.
    pub vendor_systick_config: bool,
    /// Add 1 to the highest interrupt number and specify this number in here.
    ///
    /// You can start to enumerate interrupts from 0. Gaps might exist between interrupts. For example, you have defined interrupts with the numbers 1, 2, and 8. Add 9 :(8+1) into this field.
    pub device_num_interrupts: Option<SvdConstant>,
    /// Indicate the amount of regions in the Security Attribution Unit (SAU).
    ///
    /// If the value is greater than zero, then the device has a SAU and the number indicates the maximum amount of available address regions.
    pub sau_num_regions: Option<SvdConstant>,
    /// If the Secure Attribution Unit is preconfigured by HW or Firmware, then the settings are described here.
    pub sau_regions_config: Option<SauRegionsConfigType>,
}

/// Set the configuration for the Secure Attribution Unit (SAU) when they are preconfigured by HW or Firmware.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionsConfigType {
    /// Specify whether the Secure Attribution Units are enabled.
    pub enabled: Option<bool>,
    /// Set the protection mode for disabled regions.
    ///
    /// When the complete SAU is disabled, the whole memory is treated either "s"=secure or "n"=non-secure. This value is inherited by the <region> element. Refer to element protection for details and predefined values.
    pub protection_when_disabled: Option<Protection>,
    /// Group to configure SAU regions.
    pub region: Option<Vec<SauRegionType>>,
}

/// Define the regions of the Secure Attribution Unit (SAU).
///
/// The protection level is inherited from the attribute _protectionWhenDisabled_ of the enclosing element _sauRegionsConfig_.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SauRegionType {
    /// Specify whether the Secure Attribution Units are enabled.
    pub enabled: Option<bool>,
    /// Identifiy the region with a name.
    pub name: Option<String>,
    /// Base address of the region.
    pub base: SvdConstant,
    /// Limit address of the region.
    pub limit: SvdConstant,
    /// Acces type of a region
    pub access: SauRegionAccessType,
}

/// Use one of the following predefined values to define the acces type of a region
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum SauRegionAccessType {
    /// Non secure access
    #[serde(rename = "n")]
    NonSecure,
    /// Secure callable access
    #[serde(rename = "c")]
    SecureCallable,
}

/// All peripherals of a device are enclosed within the _peripherals_ elements.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Peripherals {
    /// Define the sequence of peripherals.
    pub peripheral: Vec<Peripheral>,
}
