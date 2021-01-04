use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "String", into = "String")]
pub struct Regex {
    #[serde(skip_serializing)]
    pub regex: regex::Regex,
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.regex.as_str().to_string() == other.regex.as_str().to_string()
    }
}
impl Eq for Regex {}
impl Hash for Regex {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write(self.regex.as_str().as_bytes());
        state.finish();
    }
}

impl From<String> for Regex {
    fn from(regex_string: String) -> Self {
        Regex {
            regex: regex::Regex::new(format!("^{}$", regex_string).as_str()).unwrap(),
        }
    }
}

impl From<&str> for Regex {
    fn from(regex_string: &str) -> Self {
        Regex::from(regex_string.to_string())
    }
}

impl Into<String> for Regex {
    fn into(self) -> String {
        self.regex.as_str().to_string()
    }
}
