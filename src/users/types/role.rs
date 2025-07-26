use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::users::types::role_type::RoleType;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Role {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: RoleType,
}
