use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]

pub enum RoleType {
    USER,
    APPLICATION,
}

impl Into<String> for RoleType {
    fn into(self) -> String {
        match self {
            Self::USER => String::from("USER"),
            Self::APPLICATION => String::from("APPLICATION"),
        }
    }
}

impl From<String> for RoleType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "USER" => Self::USER,
            "APPLICATION" => Self::APPLICATION,
            _ => panic!("unknown role type"),
        }
    }
}

impl fmt::Display for RoleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::USER => write!(f, "USER"),
            Self::APPLICATION => write!(f, "APPLICATION"),
        }
    }
}
