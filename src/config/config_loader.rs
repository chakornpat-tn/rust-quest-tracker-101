use anyhow::Result;

use super::{config_models::{AdminSecret, Database, DotEnvyConfig, Server, UserSecret}, stage::Stage};
pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server:Server = Server {
        port: std::env::var("SERVER_PORT").expect("SERVER_PORT is invalid").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("SERVER_BODY_LIMIT is invalid").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("SERVER_TIMEOUT is invalid").parse()?,
    };

    let database:Database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

   Ok(DotEnvyConfig {
        server,
        database,
    })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok() ;

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_user_secret() -> Result<UserSecret> {
    dotenvy::dotenv().ok();

    Ok(UserSecret {
        secret: std::env::var("JWT_USER_SECRET").expect("JWT_USER_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_USER_REFRESH_SECRET").expect("JWT_USER_REFRESH_SECRET is invalid"),
    })
}

pub fn get_admin_secret() -> Result<AdminSecret> {
    dotenvy::dotenv().ok();

    Ok(AdminSecret {
        secret: std::env::var("JWT_ADMIN_SECRET").expect("JWT_ADMIN_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_ADMIN_REFRESH_SECRET").expect("JWT_ADMIN_REFRESH_SECRET is invalid"),
    })
}

