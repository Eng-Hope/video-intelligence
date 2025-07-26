use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::users::types::role_type::RoleType;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: RoleType,
}
