use std::fmt::{Display, Formatter};
use std::io::Error;
use std::path::Path;
use std::time::SystemTimeError;

pub mod date;
pub mod extension;
pub mod file_type;
pub mod filter_chain;
pub mod name;
pub mod owner;
pub mod permissions;
pub mod size;

pub trait FileFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError>;
}
#[derive(Debug)]
pub enum FilterError {
    IoError(Error),
    WalkdirError(walkdir::Error),
    TimeError(SystemTimeError),
    Other(String),
}

impl Display for FilterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::IoError(e) => {
                write!(f, "io error: {}", &e)
            }
            FilterError::WalkdirError(e) => {
                write!(f, "walkdir error : {}", &e)
            }
            FilterError::TimeError(e) => {
                write!(f, "time error : {}", &e)
            }
            FilterError::Other(e) => {
                write!(f, "other errors : {}", &e)
            }
        }
    }
}

impl std::error::Error for FilterError {}
