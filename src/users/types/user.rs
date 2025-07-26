use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::role_repository::get_token_by_user_id;
use crate::users::types::user_response::UserResponse;
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


impl User{
    pub async fn to_response(&self, pool: &PgPool) -> Result<UserResponse, ApplicationError> {
        Ok(UserResponse{
            id: self.id,
            name: self.name.clone(),
            email:self.email.clone(),
            role: get_token_by_user_id(&pool, &self.id).await?.role,
        }
        )
    }

}