// https://github.com/fox0/derpibooru-proxy/blob/master/src/config.rs

use std::sync::LazyLock;

use dirs::config_dir;
use serde::Deserialize;

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);

/// Конфиг приложения
#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    // pub http: Http,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "kebab-case")]
// pub struct Http {
//     pub accept: String,
//     pub accept_language: String,
//     pub cache_control: String,
//     pub cookie: String,
//     pub priority: String,
//     // pub referer: String,
//     pub sec_ch_ua: String,
//     pub sec_ch_ua_arch: String,
//     pub sec_ch_ua_bitness: String,
//     pub sec_ch_ua_full_version: String,
//     pub sec_ch_ua_full_version_list: String,
//     pub sec_ch_ua_mobile: String,
//     pub sec_ch_ua_model: String,
//     pub sec_ch_ua_platform: String,
//     pub sec_ch_ua_platform_version: String,
//     pub sec_fetch_dest: String,
//     pub sec_fetch_mode: String,
//     pub sec_fetch_site: String,
//     pub sec_fetch_user: String,
//     pub upgrade_insecure_requests: String,
//     pub user_agent: String,
// }

impl Config {
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::expect_used)]
    fn new() -> Self {
        let mut path = config_dir().unwrap();
        path.push(env!("CARGO_PKG_NAME"));
        path.set_extension("toml");

        log::info!("Read config {path:#?}");
        let content = std::fs::read_to_string(path).expect("config");
        toml::from_str(&content).unwrap()
    }
}
