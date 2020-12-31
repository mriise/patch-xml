use crate::patch_structure::reference_expression::ReferenceExpression;
use crate::xml_structure::xml_path::XmlPath;
use serde::Deserialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Deserialize, Clone)]
#[serde(from = "String", into = "String")]
pub enum ModificationType {
    Modify(ReferenceExpression),
    Replace(ReferenceExpression),
    Add(ReferenceExpression),
}

impl ModificationType {
    pub fn to_string(&self) -> String {
        match self {
            ModificationType::Modify(re) => format!("Modify[{}]", re.to_string()),
            ModificationType::Replace(re) => format!("Replace[{}]", re.to_string()),
            ModificationType::Add(re) => format!("Add[{}]", re.to_string()),
        }
    }
    pub fn get_expression<'a>(&'a self) -> &'a ReferenceExpression {
        match self {
            ModificationType::Modify(re) => &re,
            ModificationType::Replace(re) => &re,
            ModificationType::Add(re) => &re,
        }
    }
    pub fn evaluate(&self, path: &XmlPath) -> String {
        self.get_expression().evaluate(path)
    }
}

impl ModificationType {}

impl PartialEq for ModificationType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ModificationType::Modify(m1), ModificationType::Modify(m2)) => m1 == m2,
            (ModificationType::Replace(m1), ModificationType::Replace(m2)) => m1 == m2,
            (ModificationType::Add(m1), ModificationType::Add(m2)) => m1 == m2,
            (_, _) => false,
        }
    }
}
impl Eq for ModificationType {}
impl Hash for ModificationType {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.to_string().as_bytes());
        state.finish();
    }
}

impl From<String> for ModificationType {
    fn from(modification_string: String) -> Self {
        match modification_string.split_at(1) {
            ("~", pattern) => ModificationType::Replace(ReferenceExpression::from(pattern)),
            ("+", pattern) => ModificationType::Add(ReferenceExpression::from(pattern)),
            (_, _) => ModificationType::Modify(ReferenceExpression::from(modification_string)),
        }
    }
}

impl From<&str> for ModificationType {
    fn from(modification_string: &str) -> Self {
        ModificationType::from(modification_string.to_string())
    }
}

impl Into<String> for ModificationType {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify() {
        assert_eq!(
            ModificationType::from("pattern"),
            ModificationType::Modify(ReferenceExpression::from("pattern"))
        );
    }
    #[test]
    fn add() {
        assert_eq!(
            ModificationType::from("+pattern"),
            ModificationType::Add(ReferenceExpression::from("pattern"))
        );
    }
    #[test]
    fn replace() {
        assert_eq!(
            ModificationType::from("~pattern"),
            ModificationType::Replace(ReferenceExpression::from("pattern"))
        );
    }
}
