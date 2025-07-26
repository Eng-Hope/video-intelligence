use crate::application::configuration::axum_server::run;
use dotenvy::dotenv;
use env_logger;
use log::error;
use log::info;
mod application;
mod users;

#[tokio::main]
async fn main() {
    env_logger::init(); // Initialize the logger
    dotenv().ok(); //loads envs
    match run().await{
        Ok(_) => {
            info!("Application Has Started"); 
        }
        Err(err) => {
            error!("Application Error: {}", err.description);
        }
    }
}