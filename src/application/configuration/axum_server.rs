use crate::application::configuration::application_state::AppState;
use crate::application::configuration::database::initialize_database;
use crate::application::errors::application_error::ApplicationError;
use axum::{Extension, Router};
use log::error;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::users::routes::authentication::authentication;

async fn initialize_axum_server(
    listener: TcpListener,
    state: Arc<AppState>,
) -> Result<(), ApplicationError> {
    let app = Router::new()
        .nest("/auth", authentication())
        .layer(Extension(state)); //state passed here
    match axum::serve(listener, app).await {
        Ok(()) => Ok(()),
        Err(e) => {
            error!("{}", e);
            Err(ApplicationError {
                error: "Application Boot Error".into(),
                description: e.to_string(),
            })
        }
    }
}

pub async fn run() -> Result<(), ApplicationError> {
    let pool = initialize_database().await?;
    let port;

    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_) => port = String::from("0.0.0.0:8080"),
    };
    //if db initialization fails return error
    match TcpListener::bind(&port).await {
        Ok(listener) => {
            let state = Arc::new(AppState { pool });
            initialize_axum_server(listener, state).await?;
            Ok(())
        },
        Err(e) => {
            error!("{}", e.to_string());
            Err(ApplicationError {
                error: "Server Error".into(),
                description: e.to_string(),
            })
        }
    }
        //this is a shared state and can be extracted using extensions
}
