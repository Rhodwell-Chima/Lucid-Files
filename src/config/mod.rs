use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::config::config::Config;

pub mod config;

pub fn load_config_from_path(config_dir: &Path) -> Result<Config, Error> {
    let content = fs::read_to_string(config_dir)?;
    let config: Config =
        toml::from_str(&content.as_str()).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(config)
}