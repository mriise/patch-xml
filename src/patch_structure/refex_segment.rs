use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CaptureReference {
    Number(usize),
    Name(String),
    WholeExpression,
}

#[derive(Debug, Clone)]
pub struct SegmentReference {
    pub path: String,
    pub capture: CaptureReference,
}

#[derive(Debug, Clone)]
pub enum Segment {
    String(String),
    Reference(SegmentReference),
}

impl From<String> for SegmentReference {
    fn from(segment_reference_string: String) -> Self {
        let parts: Vec<&str> = segment_reference_string.split(":").collect();
        if parts.len() == 0 {
            panic!("Empty references are not allowed");
        } else if parts.len() == 1 {
            SegmentReference {
                path: parts.get(0).unwrap().to_string(),
                capture: CaptureReference::WholeExpression,
            }
        } else if parts.len() == 2 {
            let capture_reference = parts.get(1).unwrap().to_string();
            match usize::from_str(&capture_reference) {
                Ok(index) => SegmentReference {
                    path: parts.get(0).unwrap().to_string(),
                    capture: CaptureReference::Number(index),
                },
                Err(_) => SegmentReference {
                    path: parts.get(0).unwrap().to_string(),
                    capture: CaptureReference::Name(capture_reference),
                },
            }
        } else {
            panic!("A reference must have a path and optionally a capture-reference. Not more.");
        }
    }
}

impl From<&str> for SegmentReference {
    fn from(segment_reference_string: &str) -> Self {
        SegmentReference::from(segment_reference_string.to_string())
    }
}

impl Segment {
    pub fn to_string(&self) -> String {
        match self {
            Segment::String(segment) => segment.clone(),
            Segment::Reference(reference) => format!("[{}]", reference.to_string()),
        }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Segment::String(s1), Segment::String(s2)) => s1 == s2,
            (Segment::Reference(s1), Segment::Reference(s2)) => s1 == s2,
            _ => false,
        }
    }
}

impl SegmentReference {
    pub fn to_string(&self) -> String {
        match &self.capture {
            CaptureReference::Number(n) => format!("{}:{}", self.path, n).to_string(),
            CaptureReference::Name(n) => format!("{}:{}", self.path, n).to_string(),
            CaptureReference::WholeExpression => format!("{}", self.path).to_string(),
        }
    }
}

impl PartialEq for SegmentReference {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
            && match (&self.capture, &other.capture) {
                (CaptureReference::WholeExpression, CaptureReference::WholeExpression) => true,
                (CaptureReference::Name(n1), CaptureReference::Name(n2)) => n1 == n2,
                (CaptureReference::Number(n1), CaptureReference::Number(n2)) => n1 == n2,
                (_, _) => false,
            }
    }
}
