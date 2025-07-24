use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub is_enabled: bool,
    pub is_account_nn_expired: bool,
    pub is_account_non_locked: bool,
    pub password: String,
    pub image_url: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
