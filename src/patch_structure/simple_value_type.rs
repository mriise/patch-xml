use crate::patch_structure::ReferenceExpression;
use crate::xml_structure::bidirectional_xml_tree::{XmlNode, XmlNodeData};
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(untagged)]
pub enum SimpleValueType {
    Pattern(ReferenceExpression),
    Boolean(bool),
    UnsignedInteger(u64),
    SignedInteger(i64),
    Float(f64),
    Remove,
}

impl SimpleValueType {
    //ToDo: Add element as argument to avoid accidently mixups when using multiple same elements
    pub fn to_xml_node(&self, current_node: &Rc<RefCell<XmlNode>>) -> Option<XmlNodeData> {
        match self.eval_to_string(current_node) {
            Some(text) => Some(XmlNodeData::Text(text)),
            None => None,
        }
    }
    pub fn eval_to_string(&self, current_node: &Rc<RefCell<XmlNode>>) -> Option<String> {
        match self {
            SimpleValueType::Pattern(p) => Some(p.evaluate(current_node)),
            SimpleValueType::Boolean(b) => Some(b.to_string()),
            SimpleValueType::UnsignedInteger(ui) => Some(ui.to_string()),
            SimpleValueType::SignedInteger(si) => Some(si.to_string()),
            SimpleValueType::Float(fl) => Some(fl.to_string()),
            SimpleValueType::Remove => None,
        }
    }
}
