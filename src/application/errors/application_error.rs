use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
#[error("{error}: {description}")]
pub struct ApplicationError {
    pub error: String,
    pub description: String,
}
