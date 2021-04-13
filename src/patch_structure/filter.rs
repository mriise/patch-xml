use core::fmt;

use serde::de;

use crate::patch_structure::regex::Regex;
use crate::patch_structure::SimpleValueType;

pub enum FilterVariant {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Child((Regex, Box<Filter>)),
    Regex(Regex),
    Expression(Comparator, SimpleValueType),
    NotSet,
}

impl Filter {
    fn expecting(formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a pattern as string, a boolean, a (signed or unsigned) integer, again a query or an array of queries")
    }

    fn visit_bool(v: bool) -> Filter {
        Filter::Expression(Comparator::Equals, SimpleValueType::Boolean(v))
    }

    fn visit_i64(v: i64) -> Filter {
        Filter::Expression(Comparator::Equals, SimpleValueType::SignedInteger(v))
    }

    fn visit_u64(v: u64) -> Filter {
        Filter::Expression(Comparator::Equals, SimpleValueType::UnsignedInteger(v))
    }

    fn visit_str(s: &str) -> Filter {
        if s.starts_with("^") {
            return if s.ends_with("$") {
                Filter::Regex(Regex::from(&s[1..s.len() - 1]))
            } else {
                Filter::Regex(Regex::from(s.split_at(1).1))
            };
        }

        let (prefix, value) = if s.starts_with("<") {
            (Comparator::LesserThan, s.split_at(1).1)
        } else if s.starts_with("<=") {
            (Comparator::LesserEqual, s.split_at(2).1)
        } else if s.starts_with(">") {
            (Comparator::GreaterThan, s.split_at(1).1)
        } else if s.starts_with(">=") {
            (Comparator::GreaterEqual, s.split_at(2).1)
        } else if s.starts_with("=") {
            (Comparator::Equals, s.split_at(1).1)
        } else if s.starts_with("!=") {
            (Comparator::EqualsNot, s.split_at(2).1)
        } else {
            (Comparator::Equals, s)
        };
        let (prefix, value) = (prefix, serde_yaml::from_str(value).unwrap());

        Filter::Expression(prefix, value)
    }

    fn visit_unit() -> Filter {
        Filter::NotSet
    }

    fn visit_seq<'de, A: de::SeqAccess<'de>>(
        seq: &mut A,
        filter_variant: FilterVariant,
    ) -> Result<Filter, A::Error> {
        match filter_variant {
            FilterVariant::And => {
                let mut vec = vec![];
                while let Some(val) = seq.next_element()? {
                    let filter: Filter = val;
                    match filter {
                        Filter::And(children) => {
                            vec.append(&mut children.clone());
                        }
                        filter => {
                            vec.push(filter.clone());
                        }
                    };
                }
                Ok(Filter::And(vec))
            }
            FilterVariant::Or => {
                let mut vec = vec![];
                while let Some(val) = seq.next_element()? {
                    let filter: OrFilter = val;
                    match filter.filter {
                        Filter::Or(children) => {
                            vec.append(&mut children.clone());
                        }
                        filter => {
                            vec.push(filter.clone());
                        }
                    };
                }
                Ok(Filter::Or(vec))
            }
        }
    }

    fn visit_map<'de, A: de::MapAccess<'de>>(
        map: &mut A,
        filter_variant: FilterVariant,
    ) -> Result<Filter, A::Error> {
        let mut children: Vec<Filter> = Vec::new();
        while let Some(regex) = map.next_key()? {
            let regex: Regex = regex;
            let regex_str: String = regex.clone().into();
            if regex_str == "^$and$".to_string() {
                // Deserialize And....
                let filter: Filter = map.next_value()?;
                match filter_variant {
                    FilterVariant::Or => children.push(filter),
                    FilterVariant::And => match &filter {
                        Filter::And(c) => {
                            children.append(&mut c.clone());
                        }
                        f => {
                            children.push(f.clone());
                        }
                    },
                }
            } else if regex_str == "^$or$".to_string() {
                // Deserialize Or....
                let filter: OrFilter = map.next_value()?;
                match filter_variant {
                    FilterVariant::And => children.push(filter.filter),
                    FilterVariant::Or => match &filter.filter {
                        Filter::Or(c) => {
                            children.append(&mut c.clone());
                        }
                        f => {
                            children.push(f.clone());
                        }
                    },
                }
            } else {
                match filter_variant {
                    FilterVariant::And => {
                        let filter: Filter = map.next_value()?;
                        children.push(Filter::Child((regex, Box::new(filter.clone()))));
                    }
                    FilterVariant::Or => {
                        let filter: OrFilter = map.next_value()?;
                        children.push(Filter::Child((regex, Box::new(filter.filter.clone()))));
                    }
                }
            }
        }
        if children.len() == 1 {
            Ok(children.first().unwrap().clone())
        } else {
            match filter_variant {
                FilterVariant::And => Ok(Filter::And(children)),
                FilterVariant::Or => Ok(Filter::Or(children)),
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparator {
    Equals,
    EqualsNot,
    GreaterThan,
    GreaterEqual,
    LesserThan,
    LesserEqual,
}

struct OrFilter {
    filter: Filter,
}

impl<'de> de::Deserialize<'de> for OrFilter {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = OrFilter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                Filter::expecting(formatter)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(OrFilter {
                    filter: Filter::visit_bool(v),
                })
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(OrFilter {
                    filter: Filter::visit_i64(v),
                })
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(OrFilter {
                    filter: Filter::visit_u64(v),
                })
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(OrFilter {
                    filter: Filter::visit_str(s),
                })
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(OrFilter {
                    filter: Filter::visit_unit(),
                })
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                Ok(OrFilter {
                    filter: Filter::visit_seq(&mut seq, FilterVariant::Or)?,
                })
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                Ok(OrFilter {
                    filter: Filter::visit_map(&mut map, FilterVariant::Or)?,
                })
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

impl<'de> de::Deserialize<'de> for Filter {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MyVisitor;
        impl<'de> de::Visitor<'de> for MyVisitor {
            type Value = Filter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                Filter::expecting(formatter)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Filter::visit_bool(v))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Filter::visit_i64(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Filter::visit_u64(v))
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                Ok(Filter::visit_str(s))
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(Filter::visit_unit())
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                Filter::visit_seq(&mut seq, FilterVariant::And)
            }

            fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                Filter::visit_map(&mut map, FilterVariant::And)
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}
