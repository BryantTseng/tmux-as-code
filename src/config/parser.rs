use std::{collections::HashMap, fs};

use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(alias = "global")]
    pub _global_config: GlobalConfig,
    #[serde(alias = "sessions")]
    pub sessions: HashMap<String, SessionConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SessionConfig {
    #[serde(flatten)]
    pub windows: HashMap<String, WindowConfig>,
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {}

#[derive(Debug, Deserialize, Default)]
pub struct GlobalConfig {}

impl AppConfig {
    pub fn init(path: &str) -> Self {
        info!("Reading config from {}", path);

        let contents = fs::read_to_string(path).expect("Should have been able to read the file");
        let app_config: AppConfig = toml::from_str(&contents).unwrap();

        app_config
    }
}
