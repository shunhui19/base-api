use db_confiig::LogConfig;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;
use std::sync::OnceLock;

mod db_confiig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub log: LogConfig,
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
    let config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", raw_config);
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };

    crate::config::CONFIG
        .set(config)
        .expect("config should be set.");
}

pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}
