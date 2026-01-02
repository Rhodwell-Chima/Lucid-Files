// rust
use log::{error, info};
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub mod config;
use crate::filters::{
    AndMultiFilter, ExtensionFilter, FileFilter, NameFilter, NotGateFilter, OrMultiFilter,
    SizeFilter,
};
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

pub fn filter_from_config(cfg: &Filter) -> Box<dyn FileFilter> {
    match cfg {
        Filter::Extensions { allowed } => Box::new(ExtensionFilter::new(allowed.clone())),
        Filter::Sizes { min, max } => Box::new(SizeFilter::new(*min, *max)),
        Filter::Names { pattern } => Box::new(NameFilter::new(pattern.clone())),
        Filter::And { items } => {
            let v: Vec<Box<dyn FileFilter>> = items
                .iter()
                .map(|i: &Filter| filter_from_config(i))
                .collect();
            Box::new(AndMultiFilter::new(v))
        }
        Filter::Or { items } => {
            let v: Vec<Box<dyn FileFilter>> = items
                .iter()
                .map(|i: &Filter| filter_from_config(i))
                .collect();
            Box::new(OrMultiFilter::new(v))
        }
        Filter::Not { item } => {
            // `item` is `&Box<Filter>` here; `as_ref()` gives `&Filter`
            let child: Box<dyn FileFilter> = filter_from_config(item.as_ref());
            Box::new(NotGateFilter::new(child))
        }
    }
}
