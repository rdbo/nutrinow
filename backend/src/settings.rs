use serde::{Deserialize};
use config::{Config, File, ConfigError};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub host : String,
    pub port : u16,
    pub database_url : String
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let filename = if cfg!(debug_assertions) {
            "ConfigDebug.toml"
        } else {
            "Config.toml"
        };

        let cfg = Config::builder()
            .add_source(File::with_name(filename).required(true))
            .build()?;

        cfg.try_deserialize::<Settings>()
    }
}
