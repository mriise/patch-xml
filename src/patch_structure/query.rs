use crate::patch_structure::regex::Regex;
use crate::patch_structure::value::{SimpleValueType, Value};
use crate::patch_structure::Modifier;
use crate::patch_structure::ReferenceExpression;
use core::fmt;
use serde::de::Error;
use serde::{de, Deserialize};

impl From<Vec<(Regex, Query)>> for Query {
    fn from(subqueries: Vec<(Regex, Query)>) -> Self {
        Query::Complex {
            modifier: Modifier::new(),
            modification: None,
            subqueries,
        }
    }
}
impl From<Modifier> for Query {
    fn from(modifier: Modifier) -> Self {
        Query::Complex {
            modifier,
            modification: None,
            subqueries: Vec::new(),
        }
    }
}
impl From<Option<Value>> for Query {
    fn from(modification: Option<Value>) -> Self {
        Query::Complex {
            modifier: Modifier::new(),
            modification,
            subqueries: Vec::new(),
        }
    }
}
impl From<(Modifier, Option<Value>)> for Query {
    fn from(properties: (Modifier, Option<Value>)) -> Self {
        Query::Complex {
            modifier: properties.0,
            modification: properties.1,
            subqueries: Vec::new(),
        }
    }
}
impl From<(Modifier, Vec<(Regex, Query)>)> for Query {
    fn from(properties: (Modifier, Vec<(Regex, Query)>)) -> Self {
        Query::Complex {
            modifier: properties.0,
            modification: None,
            subqueries: properties.1,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Query {
    Simple(SimpleValueType),
    Complex {
        // #[serde(flatten)]
        modifier: Modifier,
        // #[serde(rename = "$modify")]
        modification: Option<Value>,
        // #[serde(flatten)]
        subqueries: Vec<(Regex, Query)>,
    },
}

enum ComplexQueryField {
    Filter,
    Move,
    Copy,
    Modification,
    Complex(Regex),
}

impl<'de> Deserialize<'de> for ComplexQueryField {
    fn deserialize<D>(deserializer: D) -> Result<ComplexQueryField, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ComplexQueryField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`complex query`")
            }

            fn visit_str<E>(self, value: &str) -> Result<ComplexQueryField, E>
            where
                E: de::Error,
            {
                match value {
                    "$if" => Ok(ComplexQueryField::Filter),
                    "$move" => Ok(ComplexQueryField::Move),
                    "$copy" => Ok(ComplexQueryField::Copy),
                    "$modify" => Ok(ComplexQueryField::Modification),
                    child => Ok(ComplexQueryField::Complex(Regex::from(child))),
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> de::Deserialize<'de> for Query {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = Query;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "remove attempt as a unit (e.g. '~'), a pattern as string, a boolean, a (signed or unsigned) integer, again a query or an array of queries")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Query::Simple(SimpleValueType::Boolean(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Query::Simple(SimpleValueType::SignedInteger(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Query::Simple(SimpleValueType::UnsignedInteger(v)))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(Query::Simple(SimpleValueType::Pattern(
                    ReferenceExpression::from(s),
                )))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Query::Simple(SimpleValueType::Remove))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut vec = vec![];
                while let Some(val) = &mut seq.next_element()? {
                    match val {
                        Query::Simple(val) => {
                            return Err(A::Error::custom(format!(
                            "It is not allowed to define a value ({:?}) to a query within an array",
                            val
                        )))
                        }
                        Query::Complex { subqueries, .. } => vec.append(subqueries), /*subqueries
                                                                                     .into_iter()
                                                                                     .map(|(k, v)| (k, v))
                                                                                     .for_each(|e| vec.push(e)),*/
                    }
                }
                Ok(Query::from(vec))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut filter = None;
                let mut move_to = None;
                let mut copy = None;
                let mut modification = None;
                let mut subqueries = Vec::new();
                while let Some(key) = map.next_key()? {
                    match key {
                        ComplexQueryField::Filter => filter = Some(map.next_value()?),
                        ComplexQueryField::Move => move_to = Some(map.next_value()?),
                        ComplexQueryField::Copy => copy = Some(map.next_value()?),
                        ComplexQueryField::Modification => {
                            if modification.is_some() {
                                return Err(de::Error::duplicate_field("$modify"));
                            }
                            modification = Some(map.next_value()?);
                        }
                        ComplexQueryField::Complex(r) => {
                            subqueries.push((r, map.next_value()?));
                        }
                    }
                }
                Ok(Query::Complex {
                    modifier: Modifier {
                        filter,
                        move_to,
                        copy,
                    },
                    modification,
                    subqueries,
                })
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}
