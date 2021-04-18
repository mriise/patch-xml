use super::{DataType, Field, ModifiedWriteValues, ReadAction, WriteConstraint};
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

/// The description of registers is the most essential part of SVD.
///
/// If the elements <size>, <access>, <resetValue>, and <resetMask> have not been specified on a higher level, then these elements are mandatory on register level.
/// A register can represent a single value or can be subdivided into individual bit-fields of specific functionality and semantics. From a schema perspective, the element <fields> is optional, however, from a specification perspective, <fields> are mandatory when they are described in the device documentation.
///
/// You can define register arrays where the single description gets duplicated automatically. The size of the array is specified by the <dim> element. Register names get composed by the element <name> and the index-specific string defined in <dimIndex>. The element <dimIncrement> specifies the address offset between two registers.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Register {
    /// Specify the register name from which to inherit data.
    ///
    /// Elements specified subsequently override inherited values.
    /// **Usage**:
    ///  - Always use the full qualifying path, which must start with the peripheral <name>, when deriving from another scope. (for example, in periperhal B, derive from peripheralA.registerX.
    ///  - You can use the register <name> only when both registers are in the same scope.
    ///  - No relative paths will work.
    ///    **Remarks**: When deriving, it is mandatory to specify at least the <name>, the <description>, and the <addressOffset>.
    pub derived_from: Option<String>,
    /// Define the number of elements in an array of registers.
    ///
    /// If <dimIncrement> is specified, this element becomes mandatory.
    pub dim: Option<SvdConstant>,
    /// Specify the address increment, in Bytes, between two neighboring registers.
    pub dim_increment: Option<SvdConstant>,
    /// Specify the substrings that replaces the %s placeholder within name and displayName.
    ///
    /// By default, the index is a decimal value starting with 0 for the first register. dimIndex should not be used together with the placeholder [%s], but rather with %s.
    pub dim_index: Option<SvdConstant>,
    /// Specify the name of the C-type structure.
    ///
    /// If not defined, then the entry of the <name> element is used.
    pub dim_name: Option<String>,
    /// Grouping element to create enumerations in the header file.
    pub dim_array_index: Option<DimArrayIndex>,
    /// String to identify the register.
    ///
    /// Register names are required to be unique within the scope of a peripheral. You can use the placeholder %s, which is replaced by the dimIndex substring. Use the placeholder [%s] only at the end of the identifier to generate arrays in the header file. The placeholder [%s] cannot be used together with dimIndex.
    pub name: String,
    /// When specified, then this string can be used by a graphical frontend to visualize the register.
    ///
    /// Otherwise the name element is displayed. displayName may contain special characters and white spaces. You can use the placeholder %s, which is replaced by the dimIndex substring. Use the placeholder [%s] only at the end of the identifier. The placeholder [%s] cannot be used together with dimIndex.
    pub display_name: String,
    /// String describing the details of the register.
    pub description: String,
    /// Specifies a group name associated with all alternate register that have the same name.
    ///
    /// At the same time, it indicates that there is a register definition allocating the same absolute address in the address space.
    pub alternate_group: Option<String>,
    /// Define an alternate register
    ///
    /// This tag can reference a register that has been defined above to current location in the description and that describes the memory location already. This tells the SVDConv's address checker that the redefinition of this particular register is intentional. The register name needs to be unique within the scope of the current peripheral. A register description is defined either for a unique address location or could be a redefinition of an already described address. In the latter case, the register can be either marked alternateRegister and needs to have a unique name, or it can have the same register name but is assigned to a register subgroup through the tag alternateGroup (specified in version 1.0).
    pub alternate_register: Option<String>,
    /// Define the address offset relative to the enclosing element.
    pub address_offset: SvdConstant,
    /// Defines the default bit-width of any register contained in the device (implicit inheritance).
    pub size: Option<SvdConstant>,
    /// Defines the default access rights for all registers.
    pub access: Option<AccessType>,
    /// Defines the protection rights for all registers.
    pub protection: Option<Protection>,
    /// Defines the default value for all registers at RESET.
    pub reset_value: Option<SvdConstant>,
    /// Identifies which register bits have a defined reset value.
    pub reset_mask: Option<SvdConstant>,
    /// Assign a specific native C datatype to a register.
    ///
    /// It can be useful to assign a specific native C datatype to a register. This helps avoiding type casts. For example, if a 32 bit register shall act as a pointer to a 32 bit unsigned data item, then dataType can be set to "uint32_t *".
    pub data_type: Option<DataType>,
    /// Element to describe the manipulation of data written to a register.
    ///
    /// If not specified, the value written to the field is the value stored in the field.
    pub modified_write_values: Option<ModifiedWriteValues>,
    /// Three mutually exclusive options exist to set write-constraints.
    pub write_constraint: Option<WriteConstraint>,
    /// If set, it specifies the side effect following a read operation.
    ///
    /// If not set, the register is not modified.
    pub read_action: Option<ReadAction>,
    /// Defines the fields of a register
    ///
    /// In case a register is subdivided into bit fields, it should be reflected in the SVD description file to create bit-access macros and bit-field structures in the header file.
    pub fields: Option<Fields>,
}

impl Register {
    /// Run a check on the register datastructure after it was loaded via Serde
    pub fn post_check(&self) -> Result<(), String> {
        match &self.fields {
            None => {}
            Some(f) => {
                for field in &f.field {
                    field.post_check()?;
                }
            }
        }
        Ok(())
    }
}

/// Grouping element to define bit-field properties of a register.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Fields {
    /// Define the bit-field properties of a register.
    pub field: Vec<Field>,
}
