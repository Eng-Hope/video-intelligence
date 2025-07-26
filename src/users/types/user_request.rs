use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String
}
