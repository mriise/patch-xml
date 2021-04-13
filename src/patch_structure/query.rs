use crate::patch_structure::regex::Regex;
use crate::patch_structure::value::ModificationValue;
use crate::patch_structure::{Modifier, SimpleValueType};
use indexmap::map::IndexMap;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ComplexQuery {
    #[serde(flatten)]
    pub modifier: Modifier,
    #[serde(rename = "$modify")]
    pub modification: Option<ModificationValue>,
    #[serde(flatten)]
    pub subqueries: IndexMap<Regex, Query>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum Query {
    Simple(SimpleValueType),
    Complex(ComplexQuery),
    ComplexVec(Vec<ComplexQuery>),
}

impl From<IndexMap<Regex, Query>> for Query {
    fn from(subqueries: IndexMap<Regex, Query>) -> Self {
        Query::Complex(ComplexQuery {
            modifier: Modifier::new(),
            modification: None,
            subqueries,
        })
    }
}
