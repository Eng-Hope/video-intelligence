use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::user_repository::get_user_by_email;
use crate::users::services::jwt_service::UserTokenResponse;
use crate::users::types::token::Token;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn persist_refresh_and_access_tokens(
    pool: &PgPool,
    tokens: &UserTokenResponse,
    email: &str,
) -> Result<UserTokenResponse, ApplicationError> {
    let mut tx = pool.begin().await?;
    let saved_user = get_user_by_email(pool, email).await?;

    //persist access token
    let access = sqlx::query_as!(
        Token,
        "insert into token(is_expired, is_revoked, id, user_id, token)
        values ($1, $2, $3, $4, $5) returning *",
        Some(false),
        Some(false),
        Uuid::new_v4(),
        saved_user.id,
        Some(&tokens.access),
    )
    .fetch_one(&mut *tx)
    .await?;

    //persist refresh token
    let refresh = sqlx::query_as!(
        Token,
        "insert into token(is_expired, is_revoked, id, user_id, token)
        values ($1, $2, $3, $4, $5) returning *",
        Some(false),
        Some(false),
        Uuid::new_v4(),
        saved_user.id,
        Some(&tokens.refresh),
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(UserTokenResponse {
        access: access.token.unwrap_or_default(),
        refresh: refresh.token.unwrap_or_default(),
    })
}

pub async fn persist_access_tokens(
    pool: &PgPool,
    access_token: &str,
    email: &str,
) -> Result<String, ApplicationError> {
    let saved_user = get_user_by_email(pool, email).await?;
    let access = sqlx::query_as!(
        Token,
        "insert into token(is_expired, is_revoked, id, user_id, token)
        values ($1, $2, $3, $4, $5) returning *",
        Some(false),
        Some(false),
        Uuid::new_v4(),
        saved_user.id,
        Some(access_token),
    )
    .fetch_one(pool)
    .await?;
    Ok(access.token.unwrap_or_default())
}

pub async fn get_token_by_token(token: &str, pool: &PgPool) -> Result<Token, ApplicationError> {
    Ok(
        sqlx::query_as!(Token, "select * from token where token.token = $1", token)
            .fetch_one(pool)
            .await?,
    )
}
