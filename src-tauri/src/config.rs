use crate::error::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref APP_CONFIG_DIR: PathBuf = {
        let config_dir = dirs::config_dir().unwrap();

        let app_config_dir = config_dir.join("bard");

        if !app_config_dir.exists() {
            fs::create_dir(&app_config_dir).unwrap();
        }

        app_config_dir
    };
    static ref CONFIG_FILE: PathBuf = APP_CONFIG_DIR.join("config.toml");
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Proxy {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

fn default_proxy_method() -> String {
    "direct".to_string()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProxyConfig {
    #[serde(default = "default_proxy_method")]
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<Proxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_proxy: Option<String>,
}

impl ProxyConfig {
    pub fn to_string(&self) -> String {
        if self.method == "proxy" {
            let proxy = self.proxy.as_ref().unwrap();
            return format!("{}{}:{}", proxy.protocol, proxy.host, proxy.port);
        }

        self.reverse_proxy.clone().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub proxy: ProxyConfig,
    pub auth_key: String,
    pub show_line_numbers: Option<bool>,
}

#[tauri::command]
pub fn read_config() -> Result<Option<Config>> {
    if !CONFIG_FILE.exists() {
        return Ok(None);
    }

    let config_str = fs::read_to_string(CONFIG_FILE.to_owned()).map_err(|e| {
        error!("failed to read configuration file: {}", e);

        e.to_string()
    })?;

    toml::from_str(&config_str).map_err(|e| {
        error!("failed to deserialize configuration: {}", e);

        e.to_string()
    })
}
