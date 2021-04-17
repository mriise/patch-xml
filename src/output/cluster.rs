use super::Register;
use crate::output::{AccessType, DimArrayIndex, Protection, SvdConstant};
use serde::Deserialize;

/// Cluster describes a sequence of neighboring registers within a peripheral.
///
/// A cluster specifies the addressOffset relative to the baseAddress of the grouping element. All register elements within a cluster specify their addressOffset relative to the cluster base address (<peripheral.baseAddress> + <cluster.addressOffset>).
// Multiple register and cluster sections may occur in any order. Since version 1.3 of the specification, the nesting of cluster elements is supported. Nested clusters express hierarchical structures of registers. It is predominantely targeted at the generation of device header files to create a C-data structure within the peripheral structure instead of a flat list of registers. Note, you can also specify an array of a cluster using the dim element.
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cluster {
    /// Specify the cluster name from which to inherit data.
    ///
    /// Elements specified subsequently override inherited values.
    /// **Usage**:
    ///  - Always use the full qualifying path, which must start with the peripheral <name>, when deriving from another scope. (for example, in periperhal B, derive from peripheralA.clusterX).
    ///  - You can use the cluster <name> when both clusters are in the same scope.
    ///  - No relative paths will work.
    ///  - Remarks: When deriving a cluster, it is mandatory to specify at least the <name>, the <description>, and the <addressOffset>.
    pub derived_from: Option<String>,
    /// Define the number of elements in an array of clusters.
    pub dim: Option<SvdConstant>,
    /// Specify the address increment, in Bytes, between two neighboring clusters of the cluster array.
    pub dim_increment: Option<SvdConstant>,
    /// Specify the strings that substitute the placeholder [%s] within the cluster <name>.
    ///
    /// Use the placeholder %s in <name> when <dimIndex> is specified.
    pub dim_index: Option<SvdConstant>,
    /// Specify the name of the C-type structure.
    ///
    /// If not defined, then the entry of the <name> element is used.
    pub dim_name: Option<String>,
    /// Grouping element to create enumerations in the header file.
    pub dim_array_index: Option<DimArrayIndex>,
    /// String to identify the cluster.
    ///
    /// Cluster names are required to be unique within the scope of a peripheral. A list of cluster names can be build using the placeholder %s. Use the placeholder [%s] at the end of the identifier to generate arrays in the header file. The placeholder [%s] cannot be used together with <dimIndex>.
    pub name: String,
    /// String describing the details of the register cluster.
    pub description: Option<String>,
    /// Specify the name of the original cluster if this cluster provides an alternative description.
    pub alternate_cluster: Option<String>,
    /// Specify the struct type name created in the device header file.
    ///
    /// If not specified, then the name of the cluster is used.
    pub header_struct_name: Option<String>,
    /// Cluster address relative to the <baseAddress> of the peripheral.
    pub address_offset: SvdConstant,
    /// Define the default bit-width of any device register (implicit inheritance).
    pub size: Option<SvdConstant>,
    /// Define access rights.
    pub access: Option<AccessType>,
    /// Specify the security privilege to access an address region.
    pub protection: Option<Protection>,
    /// Define the default value for all registers at RESET.
    pub reset_value: Option<SvdConstant>,
    /// Identify register bits that have a defined reset value.
    pub reset_mask: Option<SvdConstant>,
    /// Define a sequence of register within a cluster.
    pub register: Option<Vec<Register>>,
    /// Element to nest cluster definitions.
    pub cluster: Option<Vec<Cluster>>,
}
