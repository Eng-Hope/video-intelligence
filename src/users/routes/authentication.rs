use axum::Router;
use axum::routing::post;
use crate::users::services::authentication_service::{login, signup};

pub fn authentication() ->Router{
    Router::new()
        .route("/signup", post(signup))
        .route("/sign-in", post(login))
}