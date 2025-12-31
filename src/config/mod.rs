// rust
use log::{error, info};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub mod config;
pub use config::*;

pub fn load_config_from_path(config_path: &Path) -> Result<Config, Error> {
    let content = fs::read_to_string(config_path).map_err(|e| {
        error!("Failed to read config file {:?}: {}", config_path, e);
        Error::new(ErrorKind::Other, e)
    })?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        error!("Failed to parse config file {:?}: {}", config_path, e);
        Error::new(ErrorKind::InvalidData, e)
    })?;

    info!("Loaded configuration from {:?}", config_path);
    Ok(config)
}
