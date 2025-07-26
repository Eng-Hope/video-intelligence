use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use crate::users::types::user_source::UserSource;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_enabled: Option<bool>,
    pub is_account_non_expired: Option<bool>,
    pub is_account_non_locked: Option<bool>,
    pub password: Option<String>,
    pub image_url: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub source: UserSource,
}