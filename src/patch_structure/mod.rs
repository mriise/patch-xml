mod modification_type;
mod reference_expression;
mod refex_segment;
mod regex;

pub use crate::patch_structure::modification_type::ModificationType;
use crate::patch_structure::reference_expression::ReferenceExpression;
use crate::patch_structure::regex::Regex;
// use crate::xml_structure::xml_path::XmlPath;
use crate::xml_structure::bidirectional_xml_tree::{XmlNode, XmlNodeData};
use serde::{
    de::{self},
    Deserialize,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{error, fmt};

pub fn parse(content: &String) -> Result<Option<QueryChildType>, Box<dyn error::Error>> {
    if content.is_empty() {
        return Ok(None);
    }
    Ok(Some(serde_yaml::from_str(content)?))
}

/*fn complex_test_helper(yaml_str: &str, expected_result: QueryChildType) {
    let result: QueryChildType = serde_yaml::from_str(yaml_str).unwrap();
    assert_eq!(result, expected_result);
}*/

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Query {
    #[serde(flatten)]
    pub modifier: Modifier,
    #[serde(rename = "$modify")]
    pub modification: Option<ValueType>,
    #[serde(flatten)]
    pub subqueries: HashMap<Regex, QueryChildType>,
}

impl From<Vec<(Regex, QueryChildType)>> for Query {
    fn from(subqueries: Vec<(Regex, QueryChildType)>) -> Self {
        Query {
            modifier: Modifier::new(),
            modification: None,
            subqueries: subqueries.iter().cloned().collect(),
        }
    }
}
impl From<Modifier> for Query {
    fn from(modifier: Modifier) -> Self {
        Query {
            modifier: modifier,
            modification: None,
            subqueries: HashMap::new(),
        }
    }
}
impl From<Option<ValueType>> for Query {
    fn from(modification: Option<ValueType>) -> Self {
        Query {
            modifier: Modifier::new(),
            modification: modification,
            subqueries: HashMap::new(),
        }
    }
}
impl From<(Modifier, Option<ValueType>)> for Query {
    fn from(properties: (Modifier, Option<ValueType>)) -> Self {
        Query {
            modifier: properties.0,
            modification: properties.1,
            subqueries: HashMap::new(),
        }
    }
}
impl From<(Modifier, Vec<(Regex, QueryChildType)>)> for Query {
    fn from(properties: (Modifier, Vec<(Regex, QueryChildType)>)) -> Self {
        Query {
            modifier: properties.0,
            modification: None,
            subqueries: properties.1.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum QueryChildType {
    SimpleValue(SimpleValueType),
    QuerySet(Vec<Query>),
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Modifier {
    #[serde(rename = "$if")]
    pub filter: Option<FilterExpression>,
    #[serde(rename = "$rename", alias = "$move")]
    pub move_to: Option<ReferenceExpression>,
    #[serde(rename = "$copy")]
    pub copy: Option<ReferenceExpression>,
}

impl Modifier {
    pub fn new() -> Modifier {
        Modifier {
            filter: None,
            move_to: None,
            copy: None,
        }
    }
    pub fn is_modifying(&self) -> bool {
        self.move_to.is_some() || self.copy.is_some()
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Value {
    #[serde(flatten)]
    pub modifier: Modifier,
    #[serde(flatten)]
    pub subvalues: HashMap<ModificationType, ValueType>,
}

impl From<Vec<(ModificationType, ValueType)>> for Value {
    fn from(subvalues: Vec<(ModificationType, ValueType)>) -> Self {
        Value {
            modifier: Modifier::new(),
            subvalues: subvalues.iter().cloned().collect(),
        }
    }
}
impl From<Modifier> for Value {
    fn from(modifier: Modifier) -> Self {
        Value {
            modifier: modifier,
            subvalues: HashMap::new(),
        }
    }
}
impl From<(Modifier, Vec<(ModificationType, ValueType)>)> for Value {
    fn from(properties: (Modifier, Vec<(ModificationType, ValueType)>)) -> Self {
        Value {
            modifier: properties.0,
            subvalues: properties.1.iter().cloned().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    SimpleValue(SimpleValueType),
    ComplexValues(Vec<Value>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleValueType {
    Pattern(ReferenceExpression),
    Boolean(bool),
    UnsignedInteger(u64),
    SignedInteger(i64),
    Remove,
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

type FilterExpression = String;

impl<'de> de::Deserialize<'de> for QueryChildType {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = QueryChildType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "remove attempt as a unit (e.g. '~'), a pattern as string, a boolean, a (signed or unsigned) integer, again a query or an array of queries")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(QueryChildType::SimpleValue(SimpleValueType::Boolean(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(QueryChildType::SimpleValue(SimpleValueType::SignedInteger(
                    v,
                )))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(QueryChildType::SimpleValue(
                    SimpleValueType::UnsignedInteger(v),
                ))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(QueryChildType::SimpleValue(SimpleValueType::Pattern(
                    ReferenceExpression::from(s),
                )))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(QueryChildType::SimpleValue(SimpleValueType::Remove))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut vec = vec![];
                while let Some(val) = seq.next_element()? {
                    vec.push(val);
                }
                Ok(QueryChildType::QuerySet(vec))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                let tmp = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map));
                tmp.map(|m| QueryChildType::QuerySet(vec![m]))
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

impl<'de> de::Deserialize<'de> for ValueType {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = ValueType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "remove attempt as a unit (e.g. '~'), a pattern as string, a boolean, a (signed or unsigned) integer, a complex value or an array of complex values")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(ValueType::SimpleValue(SimpleValueType::Boolean(v)))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(ValueType::SimpleValue(SimpleValueType::SignedInteger(v)))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(ValueType::SimpleValue(SimpleValueType::UnsignedInteger(v)))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(ValueType::SimpleValue(SimpleValueType::Pattern(
                    ReferenceExpression::from(s),
                )))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(ValueType::SimpleValue(SimpleValueType::Remove))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut vec = vec![];
                while let Some(val) = seq.next_element()? {
                    vec.push(val);
                }
                Ok(ValueType::ComplexValues(vec))
            }

            fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                let tmp = de::Deserialize::deserialize(de::value::MapAccessDeserializer::new(map));
                tmp.map(|m| ValueType::ComplexValues(vec![m]))
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn complex_test_helper(yaml_str: &str, expected_result: QueryChildType) {
        let result: QueryChildType = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(result, expected_result);
    }

    mod query_test {
        use super::*;
        fn simple_value_test_helper(yaml_str: &str, simple_value_type: &SimpleValueType) {
            let result: QueryChildType = serde_yaml::from_str(yaml_str).unwrap();
            let expected_result = QueryChildType::QuerySet(vec![Query {
                modifier: Modifier::new(),
                modification: None,
                subqueries: [(
                    Regex::from("elementa"),
                    QueryChildType::SimpleValue(simple_value_type.clone()),
                )]
                .iter()
                .cloned()
                .collect(),
            }]);
            assert_eq!(result, expected_result);
        }

        #[test]
        fn test_pattern() {
            simple_value_test_helper(
                indoc! {r#"
            elementa: "hello world"
        "#},
                &SimpleValueType::Pattern(ReferenceExpression::from("hello world")),
            );
        }
        #[test]
        fn test_boolean() {
            simple_value_test_helper(
                indoc! {r#"
            elementa: true
        "#},
                &SimpleValueType::Boolean(true),
            );
        }
        #[test]
        fn test_signed_int() {
            simple_value_test_helper(
                indoc! {r#"
            elementa: -23
        "#},
                &SimpleValueType::SignedInteger(-23),
            );
        }
        #[test]
        fn test_unsigned_int() {
            simple_value_test_helper(
                indoc! {r#"
            elementa: 32
        "#},
                &SimpleValueType::UnsignedInteger(32),
            );
        }
        #[test]
        fn test_remove() {
            simple_value_test_helper(
                indoc! {r#"
            elementa: ~
        "#},
                &SimpleValueType::Remove,
            );
        }

        #[test]
        fn test_nested_queries() {
            let expected_result = QueryChildType::QuerySet(vec![Query::from(vec![(
                Regex::from("elementa"),
                QueryChildType::QuerySet(vec![Query::from(vec![(
                    Regex::from("elementb"),
                    QueryChildType::SimpleValue(SimpleValueType::Remove),
                )])]),
            )])]);
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      elementb: ~
                  "#},
                expected_result,
            );
        }

        #[test]
        fn test_query_lists() {
            let expected_result = QueryChildType::QuerySet(vec![Query::from(vec![(
                Regex::from("elementa"),
                QueryChildType::QuerySet(vec![
                    Query::from(vec![(
                        Regex::from("elementb"),
                        QueryChildType::SimpleValue(SimpleValueType::Remove),
                    )]),
                    Query::from(vec![(
                        Regex::from("elementb"),
                        QueryChildType::SimpleValue(SimpleValueType::Remove),
                    )]),
                ]),
            )])]);
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      - elementb: ~
                      - elementb: ~
                  "#},
                expected_result,
            );
        }
        #[test]
        fn test_root_query_lists() {
            let expected_result = QueryChildType::QuerySet(vec![
                Query::from(vec![(
                    Regex::from("elementa"),
                    QueryChildType::SimpleValue(SimpleValueType::Pattern(
                        ReferenceExpression::from("hello"),
                    )),
                )]),
                Query::from(vec![(
                    Regex::from("elementa"),
                    QueryChildType::SimpleValue(SimpleValueType::Pattern(
                        ReferenceExpression::from("world"),
                    )),
                )]),
            ]);
            complex_test_helper(
                indoc! {r#"
                    - elementa: hello
                    - elementa: world
                  "#},
                expected_result,
            );
        }
    }
    mod modify_test {
        use super::*;
        #[test]
        fn test_modifiers_simple() {
            let expected_result = QueryChildType::QuerySet(vec![Query::from(vec![(
                Regex::from("elementa"),
                QueryChildType::QuerySet(vec![Query::from((
                    Modifier {
                        filter: Some(String::from("pattern")),
                        move_to: Some(ReferenceExpression::from("some other place")),
                        copy: Some(ReferenceExpression::from("some place")),
                    },
                    Some(ValueType::SimpleValue(SimpleValueType::Pattern(
                        ReferenceExpression::from("hello world"),
                    ))),
                ))]),
            )])]);
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      $if: "pattern"
                      $rename: "some other place"
                      $copy: "some place"
                      $modify: "hello world"
                  "#},
                expected_result,
            );
        }
        #[test]
        fn test_modify_complex_map() {
            let expected_result = QueryChildType::QuerySet(vec![Query::from(vec![(
                Regex::from("elementa"),
                QueryChildType::QuerySet(vec![Query::from(Some(ValueType::ComplexValues(vec![
                    Value::from(vec![
                        (
                            ModificationType::from("elementb"),
                            ValueType::SimpleValue(SimpleValueType::Pattern(
                                ReferenceExpression::from("hello"),
                            )),
                        ),
                        (
                            ModificationType::from("elementc"),
                            ValueType::SimpleValue(SimpleValueType::Pattern(
                                ReferenceExpression::from("world"),
                            )),
                        ),
                    ]),
                ])))]),
            )])]);
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      $modify:
                        elementb: "hello"
                        elementc: "world"
                  "#},
                expected_result,
            );
        }
        #[test]
        fn test_modify_complex_list() {
            let expected_result = QueryChildType::QuerySet(vec![Query::from(vec![(
                Regex::from("elementa"),
                QueryChildType::QuerySet(vec![Query::from(Some(ValueType::ComplexValues(vec![
                    Value::from(vec![(
                        ModificationType::from("elementb"),
                        ValueType::SimpleValue(SimpleValueType::Pattern(
                            ReferenceExpression::from("hello"),
                        )),
                    )]),
                    Value::from(vec![(
                        ModificationType::from("elementb"),
                        ValueType::SimpleValue(SimpleValueType::Pattern(
                            ReferenceExpression::from("world"),
                        )),
                    )]),
                ])))]),
            )])]);
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      $modify:
                        - elementb: "hello"
                        - elementb: "world"
                  "#},
                expected_result,
            );
        }
    }
}
