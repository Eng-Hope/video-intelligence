use crate::application::configuration::application_state::AppState;
use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::role_repository::get_token_by_user_id;
use crate::users::services::authentication_service::authenticate_user;
use crate::users::types::user_response::UserResponse;
use crate::users::types::user_source::UserSource;
use axum::Json;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use log::error;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

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

impl User {
    pub async fn to_response(&self, pool: &PgPool) -> Result<UserResponse, ApplicationError> {
        Ok(UserResponse {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            role: get_token_by_user_id(&pool, &self.id).await?.role,
        })
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApplicationError>);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let error = Err((
            StatusCode::UNAUTHORIZED,
            Json(ApplicationError::new(
                "Authentication Error",
                "invalid token",
            )),
        ));

        //check for authorization header
        if let Some(header_value) = parts.headers.get("Authorization") {
            //parse the header to string
            if let Ok(header_str) = &header_value.to_str() {
                //check for bear token
                if header_str.starts_with("Bearer ") && header_str.len() > 7 {
                    //extract the token
                    let token = &header_str[7..];
                    //get database connection
                    return match &parts.extensions.get::<Arc<AppState>>() {
                        Some(pool) => {
                            //try to authenticate the user with token
                            match authenticate_user(Some(token), None, &pool.pool).await {
                                //return the user
                                Ok(result) => Ok(result.user),

                                Err(e) => {
                                    error!("{}", e);
                                    error
                                }
                            }
                        }
                        None => {
                            error!("Application State Not Found");
                            error
                        }
                    };
                }
            }
        }
        error
    }
}
