use std::fmt;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum UserSource {
    SYSTEM,
    GOOGLE
}

impl Into<String> for UserSource {
    fn into(self) -> String {
        match self {
            Self::SYSTEM => String::from("SYSTEM"),
            Self::GOOGLE => String::from("GOOGLE")
        }
    }
}

impl fmt::Display for UserSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserSource::SYSTEM => write!(f, "SYSTEM"),
            UserSource::GOOGLE => write!(f, "GOOGLE"),
        }
    }
}

impl From<String> for UserSource {
    fn from(value: String) -> UserSource {
        match value.as_str() {
            "SYSTEM" => Self::SYSTEM,
            "GOOGLE" => Self::GOOGLE,
            _ => panic!("Unknown user source type"),
        }
    }
}
