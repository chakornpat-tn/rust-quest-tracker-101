use std::sync::Arc;
use rust_quest_tracker::config::{config_loader, config_models::DotEnvyConfig};
use rust_quest_tracker::infrastructure::postgres::{postgres_connector};
use rust_quest_tracker::infrastructure::axum_http::{http_serve};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env:DotEnvyConfig = match config_loader::load() {
        Ok(env) => env,
       Err(e) => {
        error!("Failed to load Environment variables: {}", e);
        std::process::exit(1);
       }
    };

    info!("Env has been loaded");

    let postgres_pool =  match postgres_connector::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to establish connection to Postgres: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection pool has been established");

    http_serve::start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
        .await
        .expect("Failed to start the server");
    

}
