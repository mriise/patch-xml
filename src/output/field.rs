use super::{
    AccessType, EnumeratedValue, EnumeratedValuesUsage, ModifiedWriteValues, ReadAction,
    WriteConstraint,
};
use crate::output::{DimArrayIndex, SvdConstant};
use serde::Deserialize;

/// All fields of a register are enclosed between the <fields> opening and closing tags.
//
// A bit-field has a name that is unique within the register. The position and size within the register can be decsribed in two ways:
//
// by the combination of the least significant bit's position (lsb) and the most significant bit's position (msb), or
// the lsb and the bit-width of the field.
// A field may define an enumeratedValue in order to make the display more intuitive to read.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    /// Specify the field name from which to inherit data.
    ///
    /// Elements specified subsequently override inherited values.
    // **Usage**:
    //  - Always use the full qualifying path, which must start with the peripheral <name>, when deriving from another scope. (for example, in periperhal A and registerX, derive from peripheralA.registerYY.fieldYY.
    //  - You can use the field <name> only when both fields are in the same scope.
    //  - No relative paths will work.
    //    Remarks: When deriving, it is mandatory to specify at least the <name> and <description>.
    pub derived_from: Option<String>,
    /// Defines the number of elements in a list.
    pub dim: Option<SvdConstant>,
    /// Specify the address increment, in bits, between two neighboring list members in the address map.
    pub dim_increment: Option<SvdConstant>,
    /// Specify the strings that substitue the placeholder %s within <name> and <displayName>.
    pub dim_index: Option<SvdConstant>,
    /// Specify the name of the C-type structure.
    ///
    /// If not defined, then the entry in the <name> element is used.
    pub dim_name: Option<String>,
    /// Grouping element to create enumerations in the header file.
    pub dim_array_index: Option<DimArrayIndex>,
    /// Name string used to identify the field.
    ///
    /// Field names must be unique within a register.
    pub name: String,
    /// String describing the details of the register.
    pub description: Option<String>,
    /// Value defining the position of the least significant bit of the field within the register.
    pub bit_offset: Option<SvdConstant>,
    /// Value defining the bit-width of the bitfield within the register.
    pub bit_width: Option<SvdConstant>,
    /// Value defining the bit position of the least significant bit within the register.
    pub lsb: Option<SvdConstant>,
    /// Value defining the bit position of the most significant bit within the register.
    pub msb: Option<SvdConstant>,
    /// A string in the format: "\[<msb>:<lsb>\]"
    pub bit_range: Option<SvdConstant>,
    /// Predefined strings set the access type.
    ///
    /// The element can be omitted if access rights get inherited from parent elements.
    pub access: Option<AccessType>,
    /// Describe the manipulation of data written to a field.
    ///
    /// If not specified, the value written to the field is the value stored in the field.
    pub modified_write_values: Option<ModifiedWriteValues>,
    /// Three mutually exclusive options exist to set write-constraints.
    pub write_constraint: Option<WriteConstraint>,
    /// If set, it specifies the side effect following a read operation.
    ///
    /// If not set, the field is not modified after a read.
    pub read_action: Option<ReadAction>,
    /// Next lower level of description.
    pub enumerated_values: Option<Vec<EnumeratedValues>>,
}

/// The concept of enumerated values creates a map between unsigned integers and an identifier string.
///
/// In addition, a description string can be associated with each entry in the map.
///
///   0 <-> disabled -> "The clock source clk0 is turned off."
///   1 <-> enabled  -> "The clock source clk1 is running."
///   2 <-> reserved -> "Reserved values. Do not use."
///   3 <-> reserved -> "Reserved values. Do not use."
///
/// This information generates an enum in the device header file. The debugger may use this information to display the identifier string as well as the description. Just like symbolic constants making source code more readable, the system view in the debugger becomes more instructive. The detailed description can provide reference manual level details within the debugger.
#[derive(Debug, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumeratedValues {
    /// Makes a copy from a previously defined enumeratedValues section.
    ///
    /// No modifications are allowed. An enumeratedValues entry is referenced by its name. If the name is not unique throughout the description, it needs to be further qualified by specifying the associated field, register, and peripheral as required.
    pub derived_from: Option<String>,
    /// Identifier for the whole enumeration section.
    pub name: Option<String>,
    /// Identifier for the enumeration section.
    ///
    /// Overwrites the hierarchical enumeration type in the device header file. User is responsible for uniqueness across description.
    pub header_enum_name: Option<String>,
    /// This allows specifying two different enumerated values depending whether it is to be used for a read or a write access.
    ///
    /// If not specified, the default value read-write is used.
    pub usage: Option<EnumeratedValuesUsage>,
    /// Describes a single entry in the enumeration.
    ///
    /// The number of required items depends on the bit-width of the associated field.
    pub enumerated_value: Vec<EnumeratedValue>,
}
