use std::sync::Arc;
use axum::{Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use reqwest::StatusCode;
use uuid::Uuid;
use crate::application::configuration::application_state::AppState;
use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::authentication_repository::save_new_user_and_allocate_a_role;
use crate::users::types::user::User;
use crate::users::types::user_request::UserRequest;
use crate::users::types::user_response::UserResponse;
use crate::users::types::user_source::UserSource;

pub async fn signup(state: Extension<Arc<AppState>>, user_request: Json<UserRequest>) 
    -> Result<Json<UserResponse>, (StatusCode, Json<ApplicationError>)>
{
    if !user_request.0.password.eq(&user_request.0.confirm_password) {
        return Err((StatusCode::BAD_REQUEST, Json(ApplicationError::new("Password Miss match", "Passwords must match"))))
    }

   match  hash(&user_request.0.password, DEFAULT_COST){
       Ok(hash) => {
           let user = User{
               id: Uuid::new_v4(),
               name: user_request.0.name,
               email: user_request.0.email,
               is_enabled: Some(false),
               is_account_non_expired: Some(true),
               is_account_non_locked: Some(true),
               password: Some(hash),
               image_url: None,
               created_at: None,
               updated_at: None,
               source: UserSource::SYSTEM,
           };

           match save_new_user_and_allocate_a_role(&state.pool, &user).await {
               Ok(response) => Ok(Json(response)),
               Err(error) => {
                   log::error!("{:?}", error);
                   Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)))
               }

           }
       },
       
       Err(error) =>  Err((StatusCode::BAD_REQUEST, Json(ApplicationError::from(error))))
   }
    
}