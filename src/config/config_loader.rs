use anyhow::Result;

use super::{
    config_model::{Database, DotEnvyConfig, Secret, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_secret_env() -> Result<Secret> {
    dotenvy::dotenv().ok();

    let secret = Secret {
        secret: std::env::var("JWT_SECRET").expect("JWT_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET is invalid"),
    };

    Ok(secret)
}
