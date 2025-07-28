use crate::application::configuration::application_state::AppState;
use crate::application::errors::application_error::ApplicationError;
use crate::users::repositories::authentication_repository::save_new_user_and_allocate_a_role;
use crate::users::repositories::user_repository::get_user_by_email;
use crate::users::services::jwt_service::{
    TokenType, extract_subject, generate_persisted_access_token, generate_persisted_user_token,
    verify_token,
};
use crate::users::types::authentication_result::AuthenticationResult;
use crate::users::types::login_request::LoginRequest;
use crate::users::types::login_response::LoginResponse;
use crate::users::types::user::User;
use crate::users::types::user_request::UserRequest;
use crate::users::types::user_response::UserResponse;
use crate::users::types::user_source::UserSource;
use axum::{Extension, Json};
use bcrypt::{DEFAULT_COST, hash, verify};
use log::{error, info};
use reqwest::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use crate::users::types::access_token_response::RefreshTokenResponse;

///# Signup New User
///
/// create a new user
///
/// assign a USER role to a new user
pub async fn signup(
    state: Extension<Arc<AppState>>,
    user_request: Json<UserRequest>,
) -> Result<Json<UserResponse>, (StatusCode, Json<ApplicationError>)> {
    if !user_request.0.password.eq(&user_request.0.confirm_password) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApplicationError::new(
                "Password Miss match",
                "Passwords must match",
            )),
        ));
    }

    match hash(&user_request.0.password, DEFAULT_COST) {
        Ok(hash) => {
            let user = User {
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
        }

        Err(error) => Err((StatusCode::BAD_REQUEST, Json(ApplicationError::from(error)))),
    }
}

///# Login User
///Fetch user from db
///
///verify password
///
/// generate access and refresh token
///
/// persist tokens to db
///
/// fetch user info
///
/// return response
pub async fn login(
    state: Extension<Arc<AppState>>,
    login_request: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ApplicationError>)> {
    //get the user
    match authenticate_user(None, Some(login_request.0), &state.pool).await {
        Ok(result) => Ok(Json(result.session)),
        Err(error) => Err((StatusCode::UNAUTHORIZED, Json(error))),
    }
}

pub async fn generate_user_session(
    email: &str,
    pool: &PgPool,
) -> Result<AuthenticationResult, ApplicationError> {
    let user = get_user_by_email(pool, email).await?;
    let tokens = generate_persisted_user_token(email, pool).await?;
    let session = LoginResponse {
        access_token: tokens.access,
        refresh_token: tokens.refresh,
        user: user.to_response(pool).await?,
    };
    Ok(AuthenticationResult { session, user })
}

///# Authenticate User
///
/// If a Token is provided, it will be verified and the session will be returned
///
/// If Details (username and password) are provided, then they will be verified and the session will be retuned
///
pub async fn authenticate_user(
    token: Option<&str>,
    details: Option<LoginRequest>,
    pool: &PgPool,
) -> Result<AuthenticationResult, ApplicationError> {
    //if the token is present, then just return session using token
    if token.is_some() {
        let token = verify_token(token.unwrap_or_default(), TokenType::ACCESS, pool).await?;
        Ok(generate_user_session(&extract_subject(&token)?, pool).await?)
    }
    //verify password and username and return session
    else {
        let details = details.unwrap_or(LoginRequest {
            email: "".into(),
            password: "".into(),
        });
        match get_user_by_email(pool, &details.email).await {
            Ok(user) => {
                match verify(
                    details.password,
                    user.password.clone().unwrap_or_default().as_str(),
                ) {
                    Err(e) => {
                        log::error!("{:?}", e);
                        Err(ApplicationError::from(e))
                    }
                    Ok(result) => {
                        match result {
                            true => {
                                //generate and save access and refresh token
                                match generate_persisted_user_token(&details.email, pool).await {
                                    Ok(tokens) => {
                                        //fetch user details from db and pass them down to user response
                                        match user.to_response(pool).await {
                                            Ok(response) => {
                                                let session = LoginResponse {
                                                    access_token: tokens.access,
                                                    refresh_token: tokens.refresh,
                                                    user: response,
                                                };
                                                Ok(AuthenticationResult { session, user })
                                            }
                                            Err(error) => {
                                                error!("{:?}", error);
                                                Err(ApplicationError::generic(
                                                    "Cannot fetch user data please try again",
                                                ))
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        error!("{:?}", error);
                                        Err(ApplicationError::generic(
                                            "Cannot fetch user data please try again",
                                        ))
                                    }
                                }
                            }
                            _ => Err(ApplicationError::new(
                                "Authentication Error",
                                "Invalid Credentials.",
                            )),
                        }
                    }
                }
            }

            Err(e) => {
                error!("{:?}", e);
                Err(ApplicationError::new(
                    "Authentication Error",
                    "Invalid Credentials.",
                ))
            }
        }
    }
}

///# Refresh Token
///
/// Validate refresh token if it's return new access_token
pub async fn refresh_token(
    state: Extension<Arc<AppState>>,
    token: Json<String>,
) -> Result<Json<RefreshTokenResponse>, (StatusCode, Json<ApplicationError>)> {
    info!("{}", token.0);
    match generate_persisted_access_token(&token, &state.pool).await {
        Err(error) => {
            log::error!("{:?}", error);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error),
            ))
        }
        Ok(result) => Ok(Json(RefreshTokenResponse{ access_token: result})),
    }
}
