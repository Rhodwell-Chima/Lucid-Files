use std::io::Error;
use std::path::Path;

mod date;
mod extension;
mod file_type;
mod name;
mod owner;
mod permissions;
mod size;

pub trait FileFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError>;
}
#[derive(Debug)]
pub enum FilterError {
    IoError(Error),
    Other(String),
}
