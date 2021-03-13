use crate::patch_structure::reference_expression::ReferenceExpression;
use crate::patch_structure::{ModificationIdentifier, Modifier};
use crate::xml_structure::bidirectional_xml_tree::{XmlNode, XmlNodeData};
use core::fmt;
use serde::de::Error;
use serde::{de, Deserialize};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    SimpleValue(SimpleValueType),
    ComplexValues {
        modifier: Modifier,
        subvalues: Vec<(ModificationIdentifier, Value)>,
    },
}

impl From<Vec<(ModificationIdentifier, Value)>> for Value {
    fn from(subvalues: Vec<(ModificationIdentifier, Value)>) -> Self {
        Value::ComplexValues {
            modifier: Modifier::new(),
            subvalues: subvalues,
        }
    }
}
impl From<Modifier> for Value {
    fn from(modifier: Modifier) -> Self {
        Value::ComplexValues {
            modifier,
            subvalues: Vec::new(),
        }
    }
}

impl From<(Modifier, Vec<(ModificationIdentifier, Value)>)> for Value {
    fn from((modifier, subvalues): (Modifier, Vec<(ModificationIdentifier, Value)>)) -> Self {
        Value::ComplexValues {
            modifier,
            subvalues,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleValueType {
    Pattern(ReferenceExpression),
    Boolean(bool),
    UnsignedInteger(u64),
    SignedInteger(i64),
    Remove,
}

impl From<String> for SimpleValueType {
    fn from(pattern: String) -> Self {
        let i64_value = i64::from_str(pattern.as_str());
        let u64_value = u64::from_str(pattern.as_str());
        if pattern == "true" {
            SimpleValueType::Boolean(true)
        } else if pattern == "false" {
            SimpleValueType::Boolean(false)
        } else if u64_value.is_ok() {
            SimpleValueType::UnsignedInteger(u64_value.unwrap())
        } else if i64_value.is_ok() {
            SimpleValueType::SignedInteger(i64_value.unwrap())
        } else if pattern.is_empty() {
            SimpleValueType::Remove
        } else {
            SimpleValueType::Pattern(ReferenceExpression::parse(pattern))
        }
    }
}

impl SimpleValueType {
    //ToDo: Add element as argument to avoid accidently mixups when using multiple same elements
    pub fn to_xml_node(&self, current_node: &Rc<RefCell<XmlNode>>) -> Option<XmlNodeData> {
        match self {
            SimpleValueType::Pattern(p) => Some(XmlNodeData::Text(p.evaluate(current_node))),
            SimpleValueType::Boolean(b) => Some(XmlNodeData::Text(b.to_string())),
            SimpleValueType::UnsignedInteger(ui) => Some(XmlNodeData::Text(ui.to_string())),
            SimpleValueType::SignedInteger(si) => Some(XmlNodeData::Text(si.to_string())),
            SimpleValueType::Remove => None,
        }
    }
}

enum ComplexValueField {
    Filter,
    Move,
    Copy,
    Complex(ModificationIdentifier),
}

impl<'de> Deserialize<'de> for ComplexValueField {
    fn deserialize<D>(deserializer: D) -> Result<ComplexValueField, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ComplexValueField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`complex value`")
            }

            fn visit_str<E>(self, value: &str) -> Result<ComplexValueField, E>
            where
                E: de::Error,
            {
                match value {
                    "$if" => Ok(ComplexValueField::Filter),
                    "$move" => Ok(ComplexValueField::Move),
                    "$copy" => Ok(ComplexValueField::Copy),
                    child => Ok(ComplexValueField::Complex(ModificationIdentifier::from(
                        child,
                    ))),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "remove attempt as a unit (e.g. '~'), a pattern as string, a boolean, a (signed or unsigned) integer, a complex value or an array of complex values")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Value::SimpleValue(SimpleValueType::Boolean(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Value::SimpleValue(SimpleValueType::SignedInteger(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Value::SimpleValue(SimpleValueType::UnsignedInteger(v)))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(Value::SimpleValue(SimpleValueType::Pattern(
                    ReferenceExpression::from(s),
                )))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Value::SimpleValue(SimpleValueType::Remove))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut vec = vec![];
                while let Some(val) = &mut seq.next_element()? {
                    match val {
                        Value::SimpleValue(val) => {
                            return Err(A::Error::custom(
                                format!("It is not allowed to define a value ({:?}) to a query modification within an array", val)
                            ))
                        }
                        Value::ComplexValues { subvalues, .. } => vec.append(subvalues),
                            /*.into_iter()
                            .map(|(k, v)| (k, v))
                            .for_each(|e| vec.push(e)),*/
                    }
                }
                Ok(Value::from(vec))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter = None;
                let mut move_to = None;
                let mut copy = None;
                let mut subvalues = Vec::new();
                while let Some(key) = map.next_key()? {
                    match key {
                        ComplexValueField::Filter => filter = Some(map.next_value()?),
                        ComplexValueField::Move => move_to = Some(map.next_value()?),
                        ComplexValueField::Copy => copy = Some(map.next_value()?),
                        ComplexValueField::Complex(r) => {
                            subvalues.push((r, map.next_value()?));
                        }
                    }
                }
                Ok(Value::ComplexValues {
                    modifier: Modifier {
                        filter,
                        move_to,
                        copy,
                    },
                    subvalues,
                })
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}
