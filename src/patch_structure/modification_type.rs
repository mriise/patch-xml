use crate::patch_structure::reference_expression::ReferenceExpression;
use serde::Deserialize;
use std::hash::{Hash, Hasher};

#[derive(Debug, Deserialize, Clone)]
pub enum ModificationType {
    Modify,
    Replace,
    Add,
}

impl ModificationType {
    pub fn is_modify(&self) -> bool {
        match self {
            ModificationType::Modify => true,
            _ => false,
        }
    }
    pub fn is_replace(&self) -> bool {
        match self {
            ModificationType::Replace => true,
            _ => false,
        }
    }
    /*pub fn is_add(&self) -> bool {
        match self {
            ModificationType::Add => true,
            _ => false,
        }
    }*/
}

#[derive(Debug, Deserialize, Clone)]
#[serde(from = "String", into = "String")]
pub struct ModificationIdentifier {
    pub mod_type: ModificationType,
    pub identifier: ReferenceExpression,
}

impl ModificationIdentifier {
    pub fn to_string(&self) -> String {
        match self.mod_type {
            ModificationType::Modify => format!("Modify[{}]", self.identifier.to_string()),
            ModificationType::Replace => format!("Replace[{}]", self.identifier.to_string()),
            ModificationType::Add => format!("Add[{}]", self.identifier.to_string()),
        }
    }
    /*ToDo: pub fn get_expression<'a>(&'a self) -> &'a ReferenceExpression {
        match self {
            ModificationType::Modify(re) => &re,
            ModificationType::Replace(re) => &re,
            ModificationType::Add(re) => &re,
        }
    }
    pub fn evaluate(&self, current_node: Rc<RefCell<XmlNode>>) -> String {
        self.get_expression().evaluate(&current_node)
    }*/
}

impl ModificationIdentifier {}

impl PartialEq for ModificationIdentifier {
    fn eq(&self, other: &Self) -> bool {
        match (&self.mod_type, &other.mod_type) {
            (ModificationType::Modify, ModificationType::Modify) => {
                self.identifier == other.identifier
            }
            (ModificationType::Replace, ModificationType::Replace) => {
                self.identifier == other.identifier
            }
            (ModificationType::Add, ModificationType::Add) => self.identifier == other.identifier,
            (_, _) => false,
        }
    }
}
impl Eq for ModificationIdentifier {}
impl Hash for ModificationIdentifier {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.to_string().as_bytes());
        state.finish();
    }
}

impl From<String> for ModificationIdentifier {
    fn from(modification_string: String) -> Self {
        let (mod_type, pattern) = match modification_string.split_at(1) {
            ("~", pattern) => (ModificationType::Replace, pattern),
            ("+", pattern) => (ModificationType::Add, pattern),
            (_, _) => (ModificationType::Modify, modification_string.as_str()),
        };
        let identifier = ReferenceExpression::from(pattern);
        ModificationIdentifier {
            mod_type,
            identifier,
        }
    }
}

impl From<&str> for ModificationIdentifier {
    fn from(modification_string: &str) -> Self {
        ModificationIdentifier::from(modification_string.to_string())
    }
}

impl Into<String> for ModificationIdentifier {
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
            ModificationIdentifier::from("pattern"),
            ModificationIdentifier {
                mod_type: ModificationType::Modify,
                identifier: ReferenceExpression::from("pattern")
            }
        );
    }
    #[test]
    fn add() {
        assert_eq!(
            ModificationIdentifier::from("+pattern"),
            ModificationIdentifier {
                mod_type: ModificationType::Add,
                identifier: ReferenceExpression::from("pattern")
            }
        );
    }
    #[test]
    fn replace() {
        assert_eq!(
            ModificationIdentifier::from("~pattern"),
            ModificationIdentifier {
                mod_type: ModificationType::Replace,
                identifier: ReferenceExpression::from("pattern")
            }
        );
    }
}
