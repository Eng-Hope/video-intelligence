use sqlx::PgPool;
use crate::application::errors::application_error::ApplicationError;
use crate::users::types::user::User;

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, ApplicationError> {
    
    Ok(
        sqlx::query_as!(User, "select * from users where email = $1", email)
        .fetch_one(pool)
        .await?
    )
    
}