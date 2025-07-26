use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Token {
    pub id: Uuid,
    pub is_expired: Option<bool>,
    pub is_revoked: Option<bool>,
    pub user_id: Uuid,
    pub token: Option<String>,
    pub created_at: Option<OffsetDateTime>,
}