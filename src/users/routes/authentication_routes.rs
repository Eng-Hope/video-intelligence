use crate::users::services::authentication_service::{login, refresh_token, signup};
use crate::users::types::user::User;
use axum::Router;
use axum::middleware::from_extractor;
use axum::routing::post;

pub fn authentication() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/sign-in", post(login))
        .route(
            "/refresh",
            post(refresh_token).layer(from_extractor::<User>()),
        )
}
