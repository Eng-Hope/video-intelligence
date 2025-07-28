use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RefreshTokenResponse {
    pub access_token: String,
}