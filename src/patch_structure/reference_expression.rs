use crate::patch_structure::refex_segment::{CaptureReference, Segment, SegmentReference};
use crate::xml_structure::bidirectional_xml_tree::XmlNode;
use serde::Deserialize;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "String", into = "String")]
pub struct ReferenceExpression {
    #[serde(skip_serializing)]
    pub segments: Vec<Segment>,
}

impl ReferenceExpression {
    pub fn parse(string: String) -> ReferenceExpression {
        let mut segments = Vec::new();
        let mut buf = String::new();
        let mut escaping = false;
        let mut is_reference = false;
        for c in string.chars() {
            if escaping {
                buf.push(match c {
                    '[' => '[',
                    ']' => ']',
                    '\\' => '\\',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\'' => '\'',
                    '\"' => '\"',
                    _ => panic!("Escaped unescapable character {}", c),
                });
                escaping = false;
            } else {
                if c == '\\' {
                    escaping = true;
                } else if c == '[' || c == ']' {
                    if c == '[' {
                        if !buf.is_empty() {
                            segments.push(Segment::String(buf.clone()));
                            buf.clear();
                        }
                        is_reference = true;
                    } else {
                        if !is_reference {
                            panic!("Closing bracket without preceding opening bracket.")
                        } else {
                            is_reference = false;
                            segments.push(Segment::Reference(SegmentReference::from(buf.clone())));
                            buf.clear();
                        }
                    }
                } else {
                    buf.push(c);
                }
            }
        }
        if !buf.is_empty() {
            segments.push(Segment::String(buf.clone()));
        }
        ReferenceExpression { segments }
    }
    pub fn evaluate(&self, current_node: &Rc<RefCell<XmlNode>>) -> String {
        let mut result = String::new();
        for segment in &self.segments {
            match segment {
                Segment::String(s) => result.push_str(s),
                Segment::Reference(reference) => {
                    let splitted_path = reference.path.split("/").map(|s| s.to_string()).collect();
                    let current_node =
                        XmlNode::get_node_info_by_path(current_node.clone(), splitted_path, false);
                    let (regex, name) = match (
                        current_node.borrow().get_regex(),
                        current_node.borrow().name(),
                    ) {
                        (Some(regex), Some(name)) => (regex, name),
                        (_, _) => panic!("Could not evaluate regular expression"),
                    };
                    for capture in regex.captures_iter(&name) {
                        result.push_str(match &reference.capture {
                            CaptureReference::Number(n) => capture.get(n.clone()).unwrap().as_str(),
                            CaptureReference::Name(n) => capture.name(&n).unwrap().as_str(),
                            CaptureReference::WholeExpression => name.as_str(),
                        });
                    }
                }
            }
        }
        result
    }
    pub fn to_string(&self) -> String {
        self.segments.iter().map(Segment::to_string).collect()
    }
}

impl PartialEq for ReferenceExpression {
    fn eq(&self, other: &Self) -> bool {
        (&self.segments.len() == &other.segments.len()) &&  // zip stops at the shortest
            self.segments.iter()
                .zip(&other.segments)
                .all(|(a,b)| a == b )
    }
}

impl Eq for ReferenceExpression {}
impl Hash for ReferenceExpression {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.to_string().as_bytes());
        state.finish();
    }
}

impl From<String> for ReferenceExpression {
    fn from(string: String) -> Self {
        ReferenceExpression::parse(string)
    }
}

impl From<&str> for ReferenceExpression {
    fn from(regex_string: &str) -> Self {
        ReferenceExpression::from(regex_string.to_string())
    }
}

impl Into<String> for ReferenceExpression {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn there_and_back_again() {
        let pattern = "hello[world]lovely[myra]end".to_string();
        let refex = ReferenceExpression::parse(pattern.clone());
        assert_eq!(refex.to_string(), pattern)
    }

    #[test]
    fn common_pattern() {
        let pattern = "hello[world]lovely[myra]end".to_string();
        let refex = ReferenceExpression::parse(pattern.clone());
        assert_eq!(
            ReferenceExpression {
                segments: vec![
                    Segment::String("hello".to_string()),
                    Segment::Reference(SegmentReference::from("world")),
                    Segment::String("lovely".to_string()),
                    Segment::Reference(SegmentReference::from("myra")),
                    Segment::String("end".to_string()),
                ]
            },
            refex
        )
    }

    #[test]
    fn escaped_bracket() {
        let pattern = r#"hello[world]lovely\[myra\]end"#.to_string();
        let refex = ReferenceExpression::parse(pattern.clone());
        assert_eq!(
            ReferenceExpression {
                segments: vec![
                    Segment::String("hello".to_string()),
                    Segment::Reference(SegmentReference::from("world")),
                    Segment::String("lovely[myra]end".to_string()),
                ]
            },
            refex
        )
    }

    #[test]
    fn wrong_separation() {
        let pattern = "hello[world]lovely\\[myra\\]end".to_string();
        let refex = ReferenceExpression::parse(pattern.clone());
        assert_ne!(
            ReferenceExpression {
                segments: vec![
                    Segment::String("hello".to_string()),
                    Segment::Reference(SegmentReference::from("world")),
                    Segment::String("lovely".to_string()),
                    Segment::String("[myra]".to_string()),
                    Segment::String("end".to_string()),
                ]
            },
            refex
        )
    }

    #[test]
    fn escaped_token_pattern() {
        let pattern = r#"hello[\[\]\n\r\t\\\'\"]world\[\]\n\r\t\\\'\""#.to_string();
        let refex = ReferenceExpression::parse(pattern.clone());
        assert_eq!(
            ReferenceExpression {
                segments: vec![
                    Segment::String("hello".to_string()),
                    Segment::Reference(SegmentReference::from("[]\n\r\t\\\'\"")),
                    Segment::String("world[]\n\r\t\\\'\"".to_string()),
                ]
            },
            refex
        )
    }
}
