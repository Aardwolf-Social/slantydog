// Shared configuration for all APIs
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub backend_engine: String,
}

pub fn load_config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config.toml"))
        .build()?; // Build the Config object

    settings.try_deserialize() // Convert it to our Settings struct
}
