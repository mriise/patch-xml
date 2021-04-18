use serde::de::Error;
use serde::{de, Deserialize};
use std::num::ParseIntError;
use std::str::FromStr;

/// An enumeratedValue defines a map between an unsigned integer and a string.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValue {
    /// String describing the semantics of the value.
    /// Can be displayed instead of the value.
    pub name: String,
    /// Extended string describing the value.
    pub description: String,
    /// Defines the value of this enumerated value element
    pub value: EnumValue,
}

/// Defines the name and description for specific values
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EnumValue {
    /// Defines the name and description for all other values that are not listed explicitly.
    Default,
    /// Defines the constant for the bit-field as decimal, hexadecimal (0x...) or binary (0b... or #...) number.
    Value(SvdConstant),
}

/// Specify an address range uniquely mapped to this peripheral.
///
/// A peripheral must have at least one address block. If a peripheral is derived form another peripheral, the <addressBlock> is not mandatory.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub struct AddressBlock {
    /// Specifies the start address of an address block relative to the peripheral baseAddress.
    pub offset: SvdConstant,
    /// Specifies the number of addressUnitBits being covered by this address block.
    ///
    /// The end address of an address block results from the sum of baseAddress, offset, and (size - 1).
    pub size: SvdConstant,
    /// Defines the usage of the address block
    pub usage: AddressBlockUsage,
    /// Set the protection level for an address block.
    pub protection: Option<Protection>,
}

/// Defines the usage of the address block
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AddressBlockUsage {
    /// Registers usage
    Registers,
    /// Buffer usage
    Buffer,
    /// Reserved usage
    Reserved,
}

/// Define access rights.
///
/// Access rights can be redefined at any lower level.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum AccessType {
    /// Read access is permitted. Write operations have an undefined result.
    ReadOnly,
    /// Read operations have an undefined result. Write access is permitted.
    WriteOnly,
    /// Read and write accesses are permitted.
    ///
    /// Writes affect the state of the register and reads return the register
    ReadWrite,
    /// Read operations have an undefined results. Only the first write after reset has an effect.
    #[serde(rename = "writeOnce")]
    WriteOnce,
    /// Read access is always permitted. Only the first write access after a reset will have an effect on the content.
    ///
    /// Other write operations have an undefined result.
    #[serde(rename = "read-writeOnce")]
    ReadWriteOnce,
}

/// Element to describe the manipulation of data written to a register.
///
/// If not specified, the value written to the field is the value stored in the field.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ModifiedWriteValues {
    /// Write data bits of one shall clear (set to zero) the corresponding bit in the register.
    OneToClear,
    /// Write data bits of one shall set (set to one) the corresponding bit in the register.
    OneToSet,
    /// Write data bits of one shall toggle (invert) the corresponding bit in the register.
    OneToToggle,
    /// Write data bits of zero shall clear (set to zero) the corresponding bit in the register.
    ZeroToClear,
    /// Write data bits of zero shall set (set to one) the corresponding bit in the register.
    ZeroToSet,
    /// Write data bits of zero shall toggle (invert) the corresponding bit in the register.
    ZeroToToggle,
    /// After a write operation all bits in the field are cleared (set to zero).
    Clear,
    /// After a write operation all bits in the field are set (set to one).
    Set,
    /// After a write operation all bit in the field may be modified (default).
    Modify,
}

/// If set, it specifies the side effect following a read operation.
///
/// If not set, the register is not modified.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum ReadAction {
    /// The register is cleared (set to zero) following a read operation.
    Clear,
    /// The register is set (set to ones) following a read operation.
    Set,
    /// The register is modified in some way after a read operation.
    Modify,
    /// One or more dependent resources other than the current register are immediately affected by a read operation (it is recommended that the register description specifies these dependencies).
    ModifyExternal,
}

/// Define constraints for writing values to a field.
///
/// You can choose between three options, which are mutualy exclusive.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum WriteConstraint {
    /// Only the last read value can be written.
    WriteAsRead,
    /// Only the values listed in the enumeratedValues list can be written.
    UseEnumeratedValues,
    /// Only the values within the specified range can be written
    Range {
        /// Specify the smallest number to be written to the field.
        minimum: SvdConstant,
        /// Specify the largest number to be written to the field.
        maximum: SvdConstant,
    },
}

/// This allows specifying two different enumerated values depending whether it is to be used for a read or a write access.
///
/// If not specified, the default value read-write is used.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum EnumeratedValuesUsage {
    /// Enumerated value for reading
    Read,
    /// Enumerated value for writing
    Write,
    /// Enumerated value for reading and writing
    ReadWrite,
}

/// Specify the security privilege to access an address region.
///
/// This information is relevant for the programmer as well as the debugger when no universal access permissions have been granted. If no specific information is provided, an address region is accessible in any mode.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum Protection {
    /// Secure permission required for access
    #[serde(rename = "s")]
    Secure,
    /// Non-secure or secure permission required for access
    #[serde(rename = "n")]
    NonSecure,
    /// Privileged permission required for access
    #[serde(rename = "p")]
    Priviledged,
}

/// Name of the CPU
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
pub enum CpuNameType {
    /// Arm Cortex-M0
    CM0,
    /// Arm Cortex-M0+
    #[serde(alias = "CM0+")]
    CM0PLUS,
    /// Arm Cortex-M1
    CM1,
    /// Arm Secure Core SC000
    SC000,
    /// Arm Cortex-M23
    CM23,
    /// Arm Cortex-M3
    CM3,
    /// Arm Cortex-M33
    CM33,
    /// Arm Cortex-M35P
    CM35P,
    /// Arm Secure Core SC300
    SC300,
    /// Arm Cortex-M4
    CM4,
    /// Arm Cortex-M7
    CM7,
    /// Arm Cortex-A5
    CA5,
    /// Arm Cortex-A7
    CA7,
    /// Arm Cortex-A8
    CA8,
    /// Arm Cortex-A9
    CA9,
    /// Arm Cortex-A15
    CA15,
    /// Arm Cortex-A17
    CA17,
    /// Arm Cortex-A53
    CA53,
    /// Arm Cortex-A57
    CA57,
    /// Arm Cortex-A72
    CA72,
    /// other processor architectures
    #[serde(rename = "other")]
    Other,
}

/// Define the endianness of the processor
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum EndianType {
    /// Little endian memory (least significant byte gets allocated at the lowest address).
    Little,
    /// Byte invariant big endian data organization (most significant byte gets allocated at the lowest address).
    Big,
    /// Little and big endian are configurable for the device and become active after the next reset.
    Selectable,
    /// The endianness is neither little nor big endian.
    #[allow(non_camel_case_types)]
    other,
}

/// A peripheral can have multiple interrupts.
///
/// This entry allows the debugger to show interrupt names instead of interrupt numbers.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Interrupt {
    /// The string represents the interrupt name.
    pub name: String,
    /// The string describes the interrupt.
    pub description: Option<String>,
    /// Represents the enumeration index value associated to the interrupt.
    pub value: SvdConstant,
}

/// This information is used for generating an enum in the device header file.
///
/// The debugger may use this information to display the identifier string as well as the description. Just like symbolic constants making source code more readable, the system view in the debugger becomes more instructive.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DimArrayIndex {
    /// Specify the base name of enumerations.
    ///
    /// Overwrites the hierarchical enumeration type in the device header file. User is responsible for uniqueness across description. The headerfile generator uses the name of a peripheral or cluster as the base name for enumeration types. If <headerEnumName> element is specfied, then this string is used.
    pub header_enum_name: Option<String>,
    /// Specify the values contained in the enumeration.
    pub enumerated_value: Vec<EnumeratedValue>,
}

/// A specific native C datatype.
///
/// This helps avoiding type casts. For example, if a 32 bit register shall act as a pointer to a 32 bit unsigned data item, then dataType can be set to "uint32_t *". The following simple data types are predefined:
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    /// uint8_t: unsigned byte
    Uint8T,
    /// uint16_t: unsigned half word
    Uint16T,
    /// uint32_t: unsigned word
    Uint32T,
    /// uint64_t: unsigned double word
    Uint64T,
    /// int8_t: signed byte
    Int8T,
    /// int16_t: signed half word
    Int16T,
    /// int32_t: signed world
    Int32T,
    /// int64_t: signed double word
    Int64T,
    /// uint8_t *: pointer to unsigned byte
    #[serde(rename = "uint8_t *")]
    Uint8TPointer,
    /// uint16_t *: pointer to unsigned half word
    #[serde(rename = "uint16_t *")]
    Uint16TPointer,
    /// uint32_t *: pointer to unsigned word
    #[serde(rename = "uint32_t *")]
    Uint32TPointer,
    /// uint64_t *: pointer to unsigned double word
    #[serde(rename = "uint64_t *")]
    Uint64TPointer,
    /// int8_t *: pointer to signed byte
    #[serde(rename = "int8_t *")]
    Int8TPointer,
    /// int16_t *: pointer to signed half word
    #[serde(rename = "int16_t *")]
    Int16TPointer,
    /// int32_t *: pointer to signed world
    #[serde(rename = "int32_t *")]
    Int32TPointer,
    /// int64_t *: pointer to signed double word
    #[serde(rename = "int64_t *")]
    Int64TPointer,
}

/// Number constants shall be entered in hexadecimal, decimal, or binary format.
///
///  - The Hexadecimal format is indicated by a leading 0x.
///  - The Binary format is indicated by a leading #.
///  - All other formats are interpreted as decimal numbers.
///  - The element <enumeratedValue>.<value> can be used to define constants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SvdConstant {
    /// The constant value
    pub value: u32,
}

impl SvdConstant {
    /// Create SvdConstant from String
    ///
    /// The SvdConstant is created from a String. The accepted number formats follow the definitions in the [SVD specification](https://www.keil.com/pack/doc/CMSIS/SVD/html/svd_xml_conventions_gr.html)
    pub fn from_string(value: String) -> Result<Self, ParseIntError> {
        let result = if value.to_ascii_lowercase().starts_with("#") {
            u32::from_str_radix(&value.to_string()[1..], 2)
        } else if value.to_ascii_lowercase().starts_with("0x") {
            u32::from_str_radix(&value.to_string()[2..], 16)
        } else {
            u32::from_str(value.as_str())
        };
        match result {
            Ok(result) => Ok(SvdConstant { value: result }),
            Err(err) => Err(err),
        }
    }
}

impl<'de> de::Deserialize<'de> for SvdConstant {
    fn deserialize<D>(deserializer: D) -> Result<SvdConstant, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        SvdConstant::from_string(value).map_err(D::Error::custom)
    }
}
