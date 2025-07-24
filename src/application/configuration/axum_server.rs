use crate::application::configuration::application_state::AppState;
use crate::application::configuration::database::initialize_database;
use crate::application::errors::application_error::ApplicationError;
use axum::{Extension, Router};
use log::error;
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

async fn initialize_axum_server(
    listener: TcpListener,
    state: Arc<AppState>,
) -> Result<(), ApplicationError> {
    let app = Router::new()
        // .nest("/auth", authentication_router())
        .layer(Extension(state)); //state passed here
    if let Ok(()) = axum::serve(listener, app).await {
        Ok(())
    } else {
        let error = "Could not axum server";
        error!("{}", error);
        Err(ApplicationError {
            error: "Application Boot Error".into(),
            description: error.into(),
        })
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
    if let Ok(listener) = TcpListener::bind(&port).await {
        //this is a shared state and can be extracted using extensions
        let state = Arc::new(AppState { pool });
        initialize_axum_server(listener, state).await?;
    } else {
        let error = "Could not start server ";
        error!("{} at port {}", error, port);
        return Err(ApplicationError {
            error: "Server Error".into(),
            description: error.into(),
        });
    }
    Ok(())
}
