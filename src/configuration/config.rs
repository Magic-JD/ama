use crate::cli::command::ConfigArgs;
use serde::Deserialize;
use std::sync::OnceLock;

pub const DEFAULT_CONFIG: &str = include_str!("config.toml");
static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize, Clone)]
pub struct RawConfig {
    // No config for now
}

#[derive(Debug, Clone)]
pub struct Config {
    // No config for now
}

impl Config {
    pub fn init(config_args: ConfigArgs) {
        let config = Config::new(config_args);
        CONFIG
            .set(config)
            .expect("Configuration can only be initialized once");
    }

    fn new(_args: ConfigArgs) -> Self {
        // 1. Load default config
        let _default: RawConfig =
            toml::from_str(DEFAULT_CONFIG).expect("Default config is invalid");

        // 2. Load user config file (implement get_user_config_path)
        // let user_config: Option<RawConfig> = ...

        // 3. Consolidate
        Self {
            // No config for now
        }
    }

    /// Provides global access to the initialized configuration.
    pub fn get_config() -> &'static Config {
        CONFIG
            .get()
            .expect("Configuration has not been initialized")
    }
}
