use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[error("{error}: {description}")]
pub struct ApplicationError {
    pub error: String,
    pub description: String,
}

impl ApplicationError {
    pub fn new(error: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            description: description.into(),
        }
    }

    pub fn generic(description: impl Into<String>) -> Self {
        Self {
            error: "An error occurred".to_string(),
            description: description.into(),
        }
    }
}

// Macro to generate From implementations
macro_rules! impl_from_error {
    ($error_type:ty, $error_name:expr) => {
        impl From<$error_type> for ApplicationError {
            fn from(err: $error_type) -> Self {
                ApplicationError {
                    error: $error_name.to_string(),
                    description: err.to_string(),
                }
            }
        }
    };
}

// Use the macro to implement conversions
impl_from_error!(sqlx::Error, "Database error");
impl_from_error!(serde_json::Error, "JSON processing error");
impl_from_error!(uuid::Error, "UUID error");
impl_from_error!(std::io::Error, "IO error");
impl_from_error!(time::error::Parse, "Time parsing error");
impl_from_error!(std::num::ParseIntError, "Number parsing error");
impl_from_error!(std::num::ParseFloatError, "Float parsing error");
impl_from_error!(bcrypt::BcryptError, "Hashing Password Error");
impl_from_error!(jsonwebtoken::errors::Error, "JWT Error");
impl_from_error!(std::env::VarError, "JWT Error");

// Special cases for string types
impl From<String> for ApplicationError {
    fn from(err: String) -> Self {
        ApplicationError {
            error: "An error occurred".to_string(),
            description: err,
        }
    }
}

impl From<&str> for ApplicationError {
    fn from(err: &str) -> Self {
        ApplicationError {
            error: "An error occurred".to_string(),
            description: err.to_string(),
        }
    }
}
