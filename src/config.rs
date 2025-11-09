// https://github.com/fox0/derpibooru-proxy/blob/master/src/config.rs

use std::env;
use std::sync::LazyLock;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);

/// Конфиг приложения
/// заполняется из переменных окружения `DERPI_KEY`
#[derive(Debug)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Config {
    fn new() -> Self {
        let api_key = env::var("DERPI_KEY").ok();
        Self { api_key }
    }
}
