use crate::application::errors::application_error::ApplicationError;
use crate::users::types::role::Role;
use log::info;
use sqlx::PgPool;
use uuid::Uuid;
pub async fn get_token_by_user_id(pool: &PgPool, id: &Uuid) -> Result<Role, ApplicationError> {
    info!("id {}", id);

    Ok(
        sqlx::query_as!(Role, "select * from roles where user_id = $1", id)
            .fetch_one(pool)
            .await?,
    )
}
