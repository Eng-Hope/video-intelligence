use crate::application::errors::application_error::ApplicationError;
use log::{error, info, warn};
use sqlx::{PgPool, migrate::Migrator};
use std::env;
use std::path::Path;

///initialize database and run migrations return error for failures
pub async fn initialize_database() -> Result<PgPool, ApplicationError> {
    if let Ok(url) = env::var("DATABASE_URL") {
        if let Ok(pool) = PgPool::connect(&url).await {
            info!("Database initialized successfully.");
            //run migrations
            if let Ok(migrator) = Migrator::new(Path::new("./migrations")).await {
                if let Err(error) = migrator.run(&pool).await {
                    warn!(
                        "AN ERROR HAS OCCURRED RUNNING MIGRATION {}",
                        error.to_string()
                    );
                } else {
                    info!("MIGRATIONS APPLIED SUCCESSFUL");
                }
            } else {
                warn!("ERROR READING MIGRATION FILE");
            }
            //return connection on successes
            Ok(pool)
        } else {
            error!("DATABASE CONNECTION FAILED");
            Err(ApplicationError {
                error: "Database Connection".into(),
                description: "Database connection failed ".into(),
            })
        }
    } else {
        error!("DATABASE URL IS NOT SET");
        Err(ApplicationError {
            error: "Database url".into(),
            description: "Database url is not set ".into(),
        })
    }
}
