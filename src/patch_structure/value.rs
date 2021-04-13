use crate::patch_structure::{ModificationIdentifier, Modifier, SimpleValueType};
use indexmap::map::IndexMap;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ComplexValue {
    #[serde(flatten)]
    pub modifier: Modifier,
    #[serde(rename = "$attributes")]
    pub attributes: Option<IndexMap<String, SimpleValueType>>,
    #[serde(flatten)]
    pub subvalues: IndexMap<ModificationIdentifier, ModificationValue>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum ModificationValue {
    SimpleValue(SimpleValueType),
    ComplexValue(ComplexValue),
    ComplexValueVec(Vec<ComplexValue>),
}
