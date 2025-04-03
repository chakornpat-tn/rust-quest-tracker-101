use rust_quest_tracker::config::{config_loader, config_models::DotEnvyConfig};
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
}
