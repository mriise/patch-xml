use regex::Regex;

#[derive(Clone)]
pub struct PathSegment {
    pub name: String,
    pub regex: Regex,
}

#[derive(Clone)]
pub struct XmlPath {
    pub segments: Vec<PathSegment>,
}

impl XmlPath {
    pub fn new() -> XmlPath {
        XmlPath {
            segments: Vec::new(),
        }
    }
    pub fn apply(&mut self, path: String) {
        for segment in path.split("/") {
            match segment {
                ".." => {
                    self.segments.pop();
                }
                "." => {}
                "" => {}
                _ => panic!("Error: Individual path segments are not supported yet"),
            }
        }
    }
}
