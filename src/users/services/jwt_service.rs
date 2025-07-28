use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::token_repository::{
    get_token_by_token, persist_access_tokens, persist_refresh_and_access_tokens,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::cmp::PartialEq;
use std::env;
use chrono::{Duration, Utc};

pub fn extract_subject(token: &str) -> Result<String, ApplicationError> {
    let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref());
    let token_data = decode::<Claim>(token, &decoding_key, &Validation::new(Algorithm::HS256))?;
    Ok(token_data.claims.sub)
}

pub fn generate_token(clams: Claim) -> Result<String, ApplicationError> {
    Ok(encode(
        &Header::default(),
        &clams,
        &EncodingKey::from_secret(env::var("JWT_SECRET")?.as_ref()),
    )?)
}

pub async fn generate_persisted_user_token(
    email: &str,
    pool: &PgPool,
) -> Result<UserTokenResponse, ApplicationError> {
    
    //persist tokens to the database
    Ok(
        persist_refresh_and_access_tokens(pool, &UserTokenResponse {
        access: generate_token(get_token_claim(email, TokenType::ACCESS)?)?,
        refresh: generate_token(get_token_claim(email, TokenType::REFRESH)?)?,
    }, email).await?
    )
}

///# Save Access Token
pub async fn generate_persisted_access_token(
    token: &str,
    pg_pool: &PgPool,
) -> Result<String, ApplicationError> {
    let result = verify_token(token, TokenType::REFRESH, pg_pool).await?;
    let username = extract_subject(&result)?;
    
    Ok(persist_access_tokens(pg_pool, &generate_token(get_token_claim(&username, TokenType::ACCESS)?)?, &username).await?)
}

///# Verify JWT Token
///
/// check the token claim validity including exp
///
/// check the status of token from db such that revoked and expired token are invalid
///
/// return token if token is valid else error
pub async fn verify_token(
    token: &str,
    token_type: TokenType,
    pool: &PgPool,
) -> Result<String, ApplicationError> {
    //prepare validation
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    //validate token
    let token_data = decode::<Claim>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET")?.as_bytes()),
        &validation,
    )?;

    let claim = token_data.claims;

    //fetch token
    let token = get_token_by_token(token, pool).await?;

    //check the type
    if claim.token_type != token_type {
        return Err(ApplicationError::new(
            "JWT Token error",
            "JWT Token type mismatch",
        ));
    }

    //expiration
    if token.is_expired.unwrap_or_default() {
        return Err(ApplicationError::new(
            "JWT Token error",
            "JWT Token is expired",
        ));
    }

    //revoked
    if token.is_revoked.unwrap_or_default() {
        return Err(ApplicationError::new(
            "JWT Token error",
            "JWT Token is revoked",
        ));
    }

    //if everything is fine, then the token is valid return the token
    Ok(token.token.unwrap_or_default())
}



pub fn get_token_claim(username: &str, token_type: TokenType) -> Result<Claim, ApplicationError> {
    Ok(Claim {
        sub: username.to_owned(),
        exp: (Utc::now() + Duration::milliseconds(
            env::var(
                if token_type == TokenType::ACCESS {"JWT_ACCESS_EXPIRATION"}
                   else {"JWT_REFRESH_EXPIRATION"}
            )?
                .parse()?))
            .timestamp() as usize,
        token_type: if token_type == TokenType::ACCESS {TokenType::ACCESS.into()} else { TokenType::REFRESH.into() },
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub(crate) exp: usize,
    pub(crate) sub: String,
    pub(crate) token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenType {
    ACCESS,
    REFRESH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserTokenResponse {
    pub access: String,
    pub refresh: String,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ACCESS => write!(f, "ACCESS"),
            Self::REFRESH => write!(f, "REFRESH"),
        }
    }
}
