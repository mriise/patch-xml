use super::{AddressBlock, Cluster, Interrupt, Register};
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

/// Peripherals of a device
///
/// At least one peripheral has to be defined.
///  - Each peripheral describes all registers belonging to that peripheral.
///  - The address range allocated by a peripheral is defined through one or more address blocks.
///  - An address block and register addresses are specified relative to the base address of a peripheral. The address block information can be used for constructing a memory map for the device peripherals.
/// Starting version 1.3 of the SVD specification, arrays of peripherals can be specified. The single peripheral description gets duplicated automatically into an array. The number of array elements is specified by the <dim> element. To define arrays, the <name> needs the format myPeripheral[%s]. The tag <dimIncrement> specifies the address offset between two peripherals. To create copies of a peripheral using different names, you must use the derivedFrom attribute.
///
/// **Remarks**
/// The memory map does not contain any information about physical memory. The memory of a device is described as part of the CMSIS-Pack device description.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Peripheral {
    /// Specify the peripheral name from which to inherit data.
    ///
    /// Elements specified subsequently override inherited values.
    pub derived_from: Option<String>,
    /// Define the number of elements in an array.
    pub dim: Option<SvdConstant>,
    /// Specify the address increment, in Bytes, between two neighboring array members in the address map.
    pub dim_increment: Option<SvdConstant>,
    /// Do not define on peripheral level. By default, <dimIndex> is an integer value starting at 0.
    pub dim_index: Option<SvdConstant>,
    /// Specify the name of the C-type structure. If not defined, then the entry of the <name> element is used.
    pub dim_name: Option<String>,
    /// Grouping element to create enumerations in the header file.
    pub dim_array_index: Option<DimArrayIndex>,
    /// The string identifies the peripheral.
    ///
    /// Peripheral names are required to be unique for a device. The name needs to be an ANSI C identifier to generate the header file. You can use the placeholder [%s] to create arrays.
    pub name: String,
    /// The string specifies the version of this peripheral description.
    pub version: Option<String>,
    /// The string provides an overview of the purpose and functionality of the peripheral.
    pub description: Option<String>,
    /// All address blocks in the memory space of a device are assigned to a unique peripheral by default.
    ///
    /// If multiple peripherals describe the same address blocks, then this needs to be specified explicitly. A peripheral redefining an address block needs to specify the name of the peripheral that is listed first in the description.
    pub alternate_peripheral: Option<String>,
    /// Define a name under which the System Viewer is showing this peripheral.
    pub group_name: Option<String>,
    /// Define a string as prefix.
    /// All register names of this peripheral get this prefix.
    pub prepend_to_name: Option<String>,
    /// Define a string as suffix.
    /// All register names of this peripheral get this suffix.
    pub append_to_name: Option<String>,
    /// Specify the base name of C structures.
    /// The headerfile generator uses the name of a peripheral as the base name for the C structure type. If <headerStructName> element is specfied, then this string is used instead of the peripheral name; useful when multiple peripherals get derived and a generic type name should be used.
    pub header_struct_name: Option<String>,
    /// Define a C-language compliant logical expression returning a TRUE or FALSE result. If TRUE, refreshing the display for this peripheral is disabled and related accesses by the debugger are suppressed.
    ///
    /// Only constants and references to other registers contained in the description are allowed: <peripheral>-><register>-><field>, for example, (System->ClockControl->apbEnable == 0). The following operators are allowed in the expression [&&,||, ==, !=, >>, <<, &, |].
    /// **Attention**
    /// Use this feature only in cases where accesses from the debugger to registers of un-clocked peripherals result in severe debugging failures. SVD is intended to provide static information and does not include any run-time computation or functions. Such capabilities can be added by the tools, and is beyond the scope of this description language.
    pub disable_condition: Option<String>,
    /// Lowest address reserved or used by the peripheral.
    pub base_address: SvdConstant,
    /// Define the default bit-width of any register contained in the device (implicit inheritance).
    pub size: Option<SvdConstant>,
    /// Define default access rights for all registers.
    pub access: Option<AccessType>,
    /// Define default protection rights for all registers.
    pub protection: Option<Protection>,
    /// Define the default value for all registers at RESET.
    pub reset_value: Option<SvdConstant>,
    /// Identify which register bits have a defined reset value.
    pub reset_mask: Option<SvdConstant>,
    /// Specify an address range uniquely mapped to this peripheral.
    ///
    /// A peripheral must have at least one address block, but can allocate multiple distinct address ranges. If a peripheral is derived from another peripheral, the addressBlock is not mandatory.
    pub address_block: Option<AddressBlock>,
    /// A peripheral can have multiple associated interrupts.
    ///
    /// This entry allows the debugger to show interrupt names instead of interrupt numbers.
    pub interrupt: Option<Vec<Interrupt>>,
    /// Group to enclose register definitions.
    pub registers: Option<Registers>,
}

/// All registers of a peripheral are enclosed between the Registers element.
///
/// Clusters define a set of registers. You can either use the <cluster> or the <register> element.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Registers {
    /// Define the sequence of register clusters.
    pub cluster: Option<Vec<Cluster>>,
    /// Define the sequence of registers.
    pub register: Vec<Register>,
}
