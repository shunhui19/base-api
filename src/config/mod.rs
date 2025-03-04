pub use db_config::DbConfig;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use log_config::LogConfig;
use serde::Deserialize;
use std::sync::OnceLock;

mod db_config;
mod log_config;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub log: LogConfig,
    pub db: DbConfig,
    pub jwt: JwtConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: i64,
}

pub fn init() {
    // get raw config data.
    let raw_config = Figment::new()
        .merge(Toml::file(
            Env::var("APP_CONFIG")
                .as_deref()
                .unwrap_or("src/etc/config.toml"),
        ))
        .merge(Env::prefixed("APP_").global());
    // Deserialize raw data to ServerConfig datastruct.
    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", raw_config);
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };

    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        eprintln!("DATABASE_URL is not set.");
    }

    crate::config::CONFIG
        .set(config)
        .expect("config should be set.");
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[allow(dead_code)]
fn default_true() -> bool {
    true
}

#[allow(dead_code)]
fn default_false() -> bool {
    false
}
