use serde::{Deserialize, Serialize};
use crate::users::types::user_response::UserResponse;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserResponse
}