use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
