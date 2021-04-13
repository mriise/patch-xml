use std::error;

use serde::Deserialize;

pub use filter::{Comparator, Filter};
pub use modification_type::ModificationIdentifier;
pub use query::{ComplexQuery, Query};
pub use reference_expression::ReferenceExpression;
pub use simple_value_type::SimpleValueType;
pub use value::{ComplexValue, ModificationValue};

mod filter;
mod modification_type;
mod query;
mod reference_expression;
mod refex_segment;
mod regex;
mod simple_value_type;
mod value;

pub fn parse(content: &String) -> Result<Option<Query>, Box<dyn error::Error>> {
    if content.is_empty() {
        return Ok(None);
    }
    Ok(Some(serde_yaml::from_str(content)?))
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Modifier {
    #[serde(rename = "$if")]
    pub filter: Option<Filter>,
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

#[cfg(test)]
mod tests {
    use self::regex::Regex;
    use indexmap::indexmap;
    use indoc::indoc;

    use super::*;

    fn complex_test_helper(yaml_str: &str, expected_result: Query) {
        let result: Query = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(result, expected_result);
    }

    mod query_tests {

        use super::*;
        use crate::patch_structure::query::ComplexQuery;

        fn simple_value_test_helper(yaml_str: &str, simple_value_type: &SimpleValueType) {
            let result: Query = serde_yaml::from_str(yaml_str).unwrap();
            let expected_result = Query::Complex(ComplexQuery {
                modifier: Modifier::new(),
                modification: None,
                subqueries: [(
                    Regex::from("elementa"),
                    Query::Simple(simple_value_type.clone()),
                )]
                .iter()
                .cloned()
                .collect(),
            });
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
            let expected_result = Query::from(indexmap! {
                Regex::from("elementa") =>
                Query::from(indexmap!{ Regex::from("elementb") => Query::Simple(SimpleValueType::Remove) }),
            });
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      elementb: ~
                  "#},
                expected_result,
            );
        }

        #[test]
        fn test_query_duplicate_keys() {
            let expected_result = Query::from(indexmap! {
                Regex::from("elementa") =>
                Query::from(indexmap! {
                    Regex::from("elementb") =>
                    Query::Simple(SimpleValueType::Remove),
                    Regex::from("elementb") =>
                    Query::Simple(SimpleValueType::Remove),
                }),
            });
            complex_test_helper(
                indoc! {r#"
                    elementa:
                      elementb: ~
                      elementb: ~
                  "#},
                expected_result,
            );
        }
        #[test]
        fn test_root_query_lists() {
            let expected_result = Query::ComplexVec(vec![
                ComplexQuery {
                    modifier: Modifier {
                        filter: None,
                        move_to: None,
                        copy: None,
                    },
                    modification: None,
                    subqueries: indexmap! { Regex::from("elementa") => Query::Simple(SimpleValueType::Pattern(ReferenceExpression::from("hello"))) },
                },
                ComplexQuery {
                    modifier: Modifier {
                        filter: None,
                        move_to: None,
                        copy: None,
                    },
                    modification: None,
                    subqueries: indexmap! { Regex::from("elementa") => Query::Simple(SimpleValueType::Pattern(ReferenceExpression::from("world"))) },
                },
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
    mod filter_tests {
        use super::*;
        use indexmap::IndexMap;

        #[test]
        fn test_simple_filter() {
            let expected_result = Query::Complex(ComplexQuery {
                modifier: Modifier::new(),
                modification: None,
                subqueries: indexmap! {
                    Regex::from("elementa") =>
                    Query::Complex(ComplexQuery {
                        modifier: Modifier {
                            filter: Some(Filter::And(vec![
                                Filter::Child((
                                    Regex::from("subelement1"),
                                    Box::new(Filter::Expression(
                                        Comparator::Equals,
                                        SimpleValueType::Boolean(true),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement2"),
                                    Box::new(Filter::Expression(
                                        Comparator::GreaterThan,
                                        SimpleValueType::UnsignedInteger(4),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement3"),
                                    Box::new(Filter::Expression(
                                        Comparator::LesserThan,
                                        SimpleValueType::Float(1.0),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement4"),
                                    Box::new(Filter::Expression(
                                        Comparator::EqualsNot,
                                        SimpleValueType::SignedInteger(-2),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement5"),
                                    Box::new(Filter::Regex(Regex::from(
                                        "some(pattern)?".to_string(),
                                    ))),
                                )),
                            ])),
                            move_to: None,
                            copy: None,
                        },
                        modification: None,
                        subqueries: IndexMap::new(),
                    },
                )},
            });
            complex_test_helper(
                indoc! {r#"
                        elementa:
                            $if:
                                subelement1: =true
                                subelement2: '>4'
                                subelement3: <1.0
                                subelement4: '!=-2'
                                subelement5: '^some(pattern)?$'
                      "#},
                expected_result,
            );
        }
        #[test]
        fn test_cascaded_filter() {
            let expected_result = Query::Complex(ComplexQuery {
                modifier: Modifier::new(),
                modification: None,
                subqueries: indexmap! {
                    Regex::from("elementa") =>
                    Query::Complex(ComplexQuery {
                        modifier: Modifier {
                            filter: Some(Filter::And(vec![
                                Filter::Child((
                                    Regex::from("filter_element_a"),
                                    Box::new(Filter::And(vec![
                                        Filter::Child((
                                            Regex::from("subelement"),
                                            Box::new(Filter::Expression(
                                                Comparator::Equals,
                                                SimpleValueType::Boolean(true),
                                            )),
                                        )),
                                        Filter::Child((
                                            Regex::from("subelement"),
                                            Box::new(Filter::Expression(
                                                Comparator::GreaterThan,
                                                SimpleValueType::UnsignedInteger(4),
                                            )),
                                        )),
                                    ])),
                                )),
                                Filter::Child((
                                    Regex::from("filter_element_b"),
                                    Box::new(Filter::And(vec![
                                        Filter::Child((
                                            Regex::from("subelement"),
                                            Box::new(Filter::Expression(
                                                Comparator::Equals,
                                                SimpleValueType::Boolean(true),
                                            )),
                                        )),
                                        Filter::Child((
                                            Regex::from("subelement"),
                                            Box::new(Filter::Expression(
                                                Comparator::GreaterThan,
                                                SimpleValueType::UnsignedInteger(4),
                                            )),
                                        )),
                                    ])),
                                )),
                            ])),
                            move_to: None,
                            copy: None,
                        },
                        modification: None,
                        subqueries: IndexMap::new(),
                    })
                },
            });
            complex_test_helper(
                indoc! {r#"
                        elementa:
                            $if:
                                filter_element_a:
                                    - subelement: =true
                                    - subelement: '>4'
                                filter_element_b:
                                    - subelement: =true
                                    - subelement: '>4'
                      "#},
                expected_result.clone(),
            );
        }
        #[test]
        fn test_or_filter() {
            let expected_result = Query::Complex(ComplexQuery {
                modifier: Modifier::new(),
                modification: None,
                subqueries: indexmap! {
                    Regex::from("elementa") =>
                    Query::Complex( ComplexQuery{
                        modifier: Modifier {
                            filter: Some(Filter::And(vec![
                                Filter::Or(vec![
                                    Filter::Child((
                                        Regex::from("element0"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::UnsignedInteger(5),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("element1"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::Boolean(true),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("element2"),
                                        Box::new(Filter::Expression(
                                            Comparator::GreaterThan,
                                            SimpleValueType::Float(2.0),
                                        )),
                                    )),
                                ]),
                                Filter::Or(vec![
                                    Filter::Child((
                                        Regex::from("element"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::UnsignedInteger(5),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("element"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::UnsignedInteger(2),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("element"),
                                        Box::new(Filter::Expression(
                                            Comparator::LesserThan,
                                            SimpleValueType::UnsignedInteger(1),
                                        )),
                                    )),
                                ]),
                            ])),

                            move_to: None,
                            copy: None,
                        },
                        modification: None,
                        subqueries: IndexMap::new(),
                    }),
                },
            });
            complex_test_helper(
                indoc! {r#"
                        elementa:
                            $if:
                                - $or:
                                    $or:
                                        element0: 5
                                    element1: =true
                                    element2: '>2.0'
                                - $or:
                                    - $and:
                                        element: 5
                                    - element: =2
                                    - element: '<1'
                      "#},
                expected_result.clone(),
            );
        }
    }
    mod modify_tests {
        use value::ModificationValue;

        use super::*;
        use indexmap::IndexMap;

        #[test]
        fn test_modifiers_simple() {
            let expected_result = Query::Complex(ComplexQuery {
                modifier: Modifier::new(),
                modification: None,
                subqueries: indexmap! {
                Regex::from("elementa") =>
                    Query::Complex(ComplexQuery {
                        modifier: Modifier {
                            filter: Some(Filter::And(vec![
                                Filter::Or(vec![
                                    Filter::Child((
                                        Regex::from("subelement1"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::Pattern(ReferenceExpression::from("pattern1".to_string())),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("subelement2"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::Pattern(ReferenceExpression::from("pattern2".to_string())),
                                        )),
                                    )),
                                    Filter::Child((
                                        Regex::from("subelement3"),
                                        Box::new(Filter::Expression(
                                            Comparator::Equals,
                                            SimpleValueType::Pattern(ReferenceExpression::from("pattern3".to_string())),
                                        )),
                                    )),
                                ]),
                                Filter::Child((
                                    Regex::from("subelement4"),
                                    Box::new(Filter::Expression(
                                        Comparator::Equals,
                                        SimpleValueType::Pattern(ReferenceExpression::from("pattern4".to_string())),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement5"),
                                    Box::new(Filter::Expression(
                                        Comparator::Equals,
                                        SimpleValueType::Pattern(ReferenceExpression::from("pattern5".to_string())),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement6"),
                                    Box::new(Filter::Expression(
                                        Comparator::Equals,
                                        SimpleValueType::Pattern(ReferenceExpression::from("pattern6".to_string())),
                                    )),
                                )),
                                Filter::Child((
                                    Regex::from("subelement7"),
                                    Box::new(Filter::Expression(
                                        Comparator::Equals,
                                        SimpleValueType::Pattern(ReferenceExpression::from("pattern7".to_string())),
                                    )),
                                )),
                            ])),
                            move_to: Some(ReferenceExpression::from("some other place")),
                            copy: Some(ReferenceExpression::from("some place")),
                        },
                        modification: Some(ModificationValue::SimpleValue(
                            SimpleValueType::Pattern(ReferenceExpression::from("hello world")),
                        )),
                        subqueries: IndexMap::new(),
                    }),
                },
            });
            complex_test_helper(
                indoc! {r#"
                        elementa:
                          $if:
                              $or:
                                - subelement1: "pattern1"
                                - subelement2: "pattern2"
                                  subelement3: "pattern3"
                              $and:
                                - subelement4: "pattern4"
                                - subelement5: "pattern5"
                                  subelement6: "pattern6"
                              subelement7: "pattern7"
                          $move: "some other place"
                          $copy: "some place"
                          $modify: "hello world"
                      "#},
                expected_result,
            );
        }
        #[test]
        fn test_modify_complex_map() {
            let expected_result = Query::from(indexmap! {
                Regex::from("elementa") =>
                Query::Complex(ComplexQuery {
                    modifier: Modifier::new(),
                    modification: Some(ModificationValue::ComplexValue( ComplexValue{
                        modifier: Modifier::new(),
                        subvalues: indexmap!{
                            ModificationIdentifier::from("elementb") =>
                            ModificationValue::SimpleValue(SimpleValueType::Pattern(
                                ReferenceExpression::from("hello"),
                            )),
                            ModificationIdentifier::from("elementc") =>
                            ModificationValue::SimpleValue(SimpleValueType::Pattern(
                                ReferenceExpression::from("world"),
                            )),
                        },
                        attributes: None,
                    })),
                    subqueries: IndexMap::new(),
                }),
            });
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
            let expected_result = Query::from(indexmap! {
                Regex::from("elementa") =>
                Query::Complex (ComplexQuery{
                    modifier: Modifier::new(),
                    modification: Some(ModificationValue::ComplexValueVec(vec![
                        ComplexValue{
                            modifier: Modifier::new(),
                            subvalues: indexmap!{
                                ModificationIdentifier::from("elementb") =>
                                ModificationValue::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("hello"),
                                )),
                            },
                            attributes: None
                        },
                        ComplexValue{
                            modifier: Modifier::new(),
                            subvalues: indexmap!{
                                ModificationIdentifier::from("elementb") =>
                                ModificationValue::SimpleValue(SimpleValueType::Pattern(
                                    ReferenceExpression::from("world"),
                                )),
                            },
                            attributes: None
                        },
                    ])),
                    subqueries: IndexMap::new(),
                })
            });
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
    mod attribute_tests {
        use value::ModificationValue;

        use super::*;
        use indexmap::IndexMap;

        #[test]
        fn test_simple_attributes_modification() {
            let expected_result = Query::from(indexmap! {
                Regex::from("elementa") =>
                Query::Complex( ComplexQuery {
                    modifier: Modifier::new(),
                    modification: Some(ModificationValue::ComplexValue(ComplexValue {
                        modifier: Modifier::new(),
                        subvalues: IndexMap::new(),
                        attributes: Some(indexmap!{
                            "attribute1".to_string() =>
                            SimpleValueType::Pattern(ReferenceExpression::from("hello")),
                            "attribute2".to_string() => SimpleValueType::Remove
                        }),
                    })),
                    subqueries: IndexMap::new(),
                }),
            });
            complex_test_helper(
                indoc! {r#"
                        elementa:
                          $modify:
                            $attributes:
                              attribute1: "hello"
                              attribute2: ~
                      "#},
                expected_result,
            );
        }
    }
}
