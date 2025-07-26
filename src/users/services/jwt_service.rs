use std::cmp::PartialEq;
use std::env;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::token_repository::{get_token_by_token, persist_refresh_and_access_tokens};

pub fn extract_subject(token: &str) -> Result<String, ApplicationError> {
    let decoding_key = DecodingKey::from_secret(env::var("JWT_SECRET")?.as_ref());
    let token_data = decode::<Claim>(token, &decoding_key, &Validation::new(Algorithm::HS256))?;
    Ok(token_data.claims.sub)
}


pub fn generate_token(clams: Claim) -> Result<String, ApplicationError> {
    Ok(
        encode(
            &Header::default(),
            &clams,
            &EncodingKey::from_secret(
                env::var("JWT_SECRET")?
                    .as_ref(),
            ),
        )?
    )
}


pub async fn
generate_persisted_user_token(email: &str, pool: &PgPool) ->Result<UserTokenResponse, ApplicationError>{
    let access_token_claims = Claim {
        sub: email.to_owned(),
        exp: env::var("JWT_ACCESS_EXPIRATION")?.parse()?,
        token_type: TokenType::ACCESS.into(),
    };

    let refresh_token_claims = Claim {
        sub: email.to_owned(),
        exp: env::var("JWT_REFRESH_EXPIRATION")?.parse()?,
        token_type: TokenType::REFRESH.into(),
    };
    //generate tokens
    let access_token = generate_token(access_token_claims)?;
    let refresh_token = generate_token(refresh_token_claims)?;
    //persist tokens to the database

    let tokens_to_persist = UserTokenResponse{
        access: access_token,
        refresh: refresh_token,
    };

    Ok(persist_refresh_and_access_tokens(pool, &tokens_to_persist, email).await?)
}


///# Verify JWT Token
///
/// check the token claim validity including exp
///
/// check the status of token from db such that revoked and expired token are invalid
///
pub async fn verify_token(token: &str, token_type: TokenType, pool: &PgPool) -> Result<String, ApplicationError>{

    //prepare validation
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    //validate token
    let token_data = decode::<Claim>(
        token,
        &DecodingKey::from_secret( env::var("JWT_SECRET")?.as_bytes()),
        &validation,
    )?;

    let claim = token_data.claims;

    //fetch token
    let token = get_token_by_token(token, pool).await?;

    //check the type
    if claim.token_type != token_type{
        return Err(ApplicationError::new("JWT Token error", "JWT Token type mismatch"))
    }

    //expiration
    if token.is_expired.unwrap_or_default(){
        return Err(ApplicationError::new("JWT Token error", "JWT Token is expired"))
    }

    //revoked
    if  token.is_revoked.unwrap_or_default(){
        return Err(ApplicationError::new("JWT Token error", "JWT Token is revoked"))
    }

    //if everything is fine, then the token is valid return the token
    Ok(token.token.unwrap_or_default())

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
pub struct  UserTokenResponse {
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


