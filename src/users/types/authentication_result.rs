use crate::users::types::login_response::LoginResponse;
use crate::users::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuthenticationResult {
    pub session: LoginResponse,
    pub user: User,
}
