use std::io::Error;
use std::path::Path;
use std::time::SystemTimeError;

mod date;
mod extension;
mod file_type;
mod name;
mod owner;
mod permissions;
mod size;
mod filter_chain;

pub trait FileFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError>;
}
#[derive(Debug)]
pub enum FilterError {
    IoError(Error),
    TimeError(SystemTimeError),
    Other(String),
}
