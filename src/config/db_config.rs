use super::default_false;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    #[serde(alias = "database_url")]
    pub url: String,
    #[serde(default = "defautl_pool_size")]
    pub pool_size: u32,
    pub min_idle: Option<u32>,

    #[serde(default = "default_tcp_timeout")]
    pub tcp_timeout: u64,
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
    #[serde(default = "default_statement_timeout")]
    pub statement_timeout: u64,
    #[serde(default = "default_helper_threads")]
    pub helper_threads: usize,
    #[serde(default = "default_false")]
    pub enforce_tls: bool,
}

fn default_helper_threads() -> usize {
    10
}

fn defautl_pool_size() -> u32 {
    10
}

fn default_tcp_timeout() -> u64 {
    10000
}

fn default_connection_timeout() -> u64 {
    30000
}

fn default_statement_timeout() -> u64 {
    30000
}
