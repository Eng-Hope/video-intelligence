use crate::application::errors::application_error::ApplicationError;
use crate::users::types::role::Role;
use crate::users::types::role_type::RoleType;
use crate::users::types::user::User;
use crate::users::types::user_response::UserResponse;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn save_new_user_and_allocate_a_role(
    pool: &PgPool,
    user: &User,
) -> Result<UserResponse, ApplicationError> {
    let mut tx = pool.begin().await?;

    let saved_user = sqlx::query_as!(
        User,
        "insert into users (id, name, email, is_enabled, is_account_non_expired,
                   is_account_non_locked, password, image_url, source)
          VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *",
        &user.id,
        &user.name,
        &user.email,
        user.is_enabled,
        user.is_account_non_expired,
        user.is_account_non_locked,
        user.password.as_deref(),
        user.image_url.as_deref(),
        &user.source.to_string()
    )
    .fetch_one(&mut *tx)
    .await?;

    let saved_role = sqlx::query_as!(
        Role,
        "insert into roles(id, user_id, role) values ($1, $2, $3) returning *",
        Uuid::new_v4(),
        &saved_user.id,
        RoleType::USER.to_string()
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok({
        UserResponse {
            id: saved_user.id,
            name: saved_user.name,
            email: saved_user.email,
            role: saved_role.role,
        }
    })
}
