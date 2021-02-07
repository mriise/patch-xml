mod modification_type;
pub mod reference_expression;
mod refex_segment;
pub mod regex;

pub use crate::patch_structure::modification_type::ModificationIdentifier;
use crate::patch_structure::reference_expression::ReferenceExpression;
use crate::patch_structure::regex::Regex;
use crate::xml_structure::bidirectional_xml_tree::{XmlNode, XmlNodeData};
use serde::de::Error;
use serde::{
    de::{self},
    Deserialize,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{error, fmt};

pub fn parse(content: &String) -> Result<Option<Query>, Box<dyn error::Error>> {
    if content.is_empty() {
        return Ok(None);
    }
    Ok(Some(serde_yaml::from_str(content)?))
}

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

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Modifier {
    #[serde(rename = "$if")]
    pub filter: Option<String>,
    #[serde(rename = "$move")]
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
    /*pub fn is_modifying(&self) -> bool {
        self.move_to.is_some() || self.copy.is_some()
    }*/
}

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
    fn from(properties: (Modifier, Vec<(ModificationIdentifier, Value)>)) -> Self {
        Value::ComplexValues {
            modifier: properties.0,
            subvalues: properties.1,
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

/*#[derive(Debug, PartialEq, Clone)]
pub enum Filter {
    // A feature in the future: Consecutive subelements can be filtered with arrays
    /*Ordered(Vec<Filter>),*/
    And(HashMap<Regex, Filter>),
    Or(HashMap<Regex, Filter>),
    Regex(Regex),
    Expression(Comparator, SimpleValueType),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparator {
    Equals,
    GreaterThan,
    GreaterEqual,
    LesserThan,
    LesserEqual,
}
*/
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
                formatter.write_str("`secs` or `nanos`")
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
                while let Some(val) = seq.next_element()? {
                    match val {
                        Query::Simple(val) => {
                            return Err(A::Error::custom(format!(
                            "It is not allowed to define a value ({:?}) to a query within an array",
                            val
                        )))
                        }
                        Query::Complex { subqueries, .. } => subqueries
                            .into_iter()
                            .map(|(k, v)| (k, v))
                            .for_each(|e| vec.push(e)),
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
                formatter.write_str("`secs` or `nanos`")
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
                while let Some(val) = seq.next_element()? {
                    match val {
                        Value::SimpleValue(val) => {
                            return Err(A::Error::custom(
                                format!("It is not allowed to define a value ({:?}) to a query modification within an array", val)
                            ))
                        }
                        Value::ComplexValues { subvalues, .. } => subvalues
                            .into_iter()
                            .map(|(k, v)| (k, v))
                            .for_each(|e| vec.push(e)),
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

/*impl<'de> de::Deserialize<'de> for Filter {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = Filter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                //ToDo: Correct warning...
                write!(formatter, "a pattern as string, a boolean, a (signed or unsigned) integer, again a query or an array of queries")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Filter::Expression(
                    Comparator::Equals,
                    SimpleValueType::Boolean(v),
                ))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Filter::Expression(
                    Comparator::Equals,
                    SimpleValueType::SignedInteger(v),
                ))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(FilterType::Expression(
                    Comparator::Equals,
                    SimpleValueType::UnsignedInteger(v),
                ))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                if s == "and" {

                } else if s.starts_with("=") {
                    let simple_value = SimpleValueType::deserialize();
                    let x = de::Deserialize::deserialize(de::value::StringDeserializer(
                        s.split_at(1).1.to_string(),
                    ));
                    x.map(|x|FilterType::Expression(Comparator::Equals, x))
                } else {
                    //ToDo: Fill all possible kinds of filtertype expressions
                    FilterType::
                }

                /*Ok(QueryChildType::SimpleValue(SimpleValueType::Pattern(
                    ReferenceExpression::from(s),
                )))*/
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
*/

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn complex_test_helper(yaml_str: &str, expected_result: Query) {
        let result: Query = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(result, expected_result);
    }

    mod query_test {
        use super::*;
        fn simple_value_test_helper(yaml_str: &str, simple_value_type: &SimpleValueType) {
            let result: Query = serde_yaml::from_str(yaml_str).unwrap();
            let expected_result = Query::Complex {
                modifier: Modifier::new(),
                modification: None,
                subqueries: [(
                    Regex::from("elementa"),
                    Query::Simple(simple_value_type.clone()),
                )]
                .iter()
                .cloned()
                .collect(),
            };
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
            let expected_result = Query::from(vec![(
                Regex::from("elementa"),
                Query::from(vec![(
                    Regex::from("elementb"),
                    Query::Simple(SimpleValueType::Remove),
                )]),
            )]);
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
            let expected_result = Query::from(vec![(
                Regex::from("elementa"),
                Query::from(vec![
                    (
                        Regex::from("elementb"),
                        Query::Simple(SimpleValueType::Remove),
                    ),
                    (
                        Regex::from("elementb"),
                        Query::Simple(SimpleValueType::Remove),
                    ),
                ]),
            )]);
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
            let expected_result = Query::from(vec![
                (
                    Regex::from("elementa"),
                    Query::Simple(SimpleValueType::Pattern(ReferenceExpression::from("hello"))),
                ),
                (
                    Regex::from("elementa"),
                    Query::Simple(SimpleValueType::Pattern(ReferenceExpression::from("world"))),
                ),
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
        /*fn test_modifiers_simple() {
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
                      $if:
                        subelement1: "pattern1"
                        subelement2: "pattern2"
                      $move: "some other place"
                      $copy: "some place"
                      $modify: "hello world"
                  "#},
                expected_result,
            );
            r#"
                    elementa:
                      $if:
                        $or
                          subelementOr1: "pattern1"
                          subelementOr2:
                            subsubelementOr2And1: "pattern2"
                            subsubelementOr2And2: "pattern3"
                        $and
                          subelementAnd1: "pattern3"
                      $move: "some other place"
                      $copy: "some place"
                      $modify: "hello world"
                  "#;
            r#"
                    elementa:
                      $if:
                        $reg: "regular_expression"
                      $move: "some other place"
                      $copy: "some place"
                      $modify: "hello world"
                  "#;
            r#"
                    elementa:
                      $if:
                        - subelement1: "pattern1"
                        -
                          subelement2a: "pattern2"
                          $or:
                            subelement2ba: "pattern3"
                            subelement2bb: "pattern4"
                      $move: "some other place"
                      $copy: "some place"
                      $modify: "hello world"
                  "#;
        }*/
        #[test]
        fn test_modify_complex_map() {
            let expected_result = Query::from(vec![(
                Regex::from("elementa"),
                Query::Complex {
                    modifier: Modifier::new(),
                    modification: Some(Value::ComplexValues {
                        modifier: Modifier::new(),
                        subvalues: vec![
                            (
                                ModificationIdentifier::from("elementb"),
                                Value::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("hello"),
                                )),
                            ),
                            (
                                ModificationIdentifier::from("elementc"),
                                Value::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("world"),
                                )),
                            ),
                        ],
                    }),
                    subqueries: vec![],
                },
            )]);
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
            let expected_result = Query::from(vec![(
                Regex::from("elementa"),
                Query::Complex {
                    modifier: Modifier::new(),
                    modification: Some(Value::ComplexValues {
                        modifier: Modifier {
                            filter: None,
                            move_to: None,
                            copy: None,
                        },
                        subvalues: vec![
                            (
                                ModificationIdentifier::from("elementb"),
                                Value::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("hello"),
                                )),
                            ),
                            (
                                ModificationIdentifier::from("elementb"),
                                Value::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("world"),
                                )),
                            ),
                        ],
                    }),
                    subqueries: vec![],
                },
            )]);
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
