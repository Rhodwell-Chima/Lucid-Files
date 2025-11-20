pub mod date;
pub mod extension;
pub mod name;
pub mod size;

pub use date::*;
pub use extension::*;
pub use name::*;
pub use size::*;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

pub trait FileClassifier {
    fn classify(&self, path: &Path) -> Result<Option<String>, ClassifierError>;
}

#[derive(Debug)]
pub enum ClassifierError {
    InvalidFileName,
    MissingExtension,
    InvalidExtensionEncoding,
    Io(std::io::Error),
    InvalidRange(u64, u64),
    NotAFile,
    TimeConversion,
    InvalidDateRange,
}

impl Display for ClassifierError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassifierError::InvalidFileName => {
                write!(f, "filename is not valid UTF-8")
            }
            ClassifierError::MissingExtension => {
                write!(f, "file has no extension")
            }
            ClassifierError::InvalidExtensionEncoding => {
                write!(f, "extension is not valid UTF-8")
            }
            ClassifierError::Io(e) => {
                write!(f, "I/O classification error: {}", e)
            }
            ClassifierError::InvalidRange(min, max) => {
                write!(f, "Invalid Size Range: {} {}", min, max)
            }
            ClassifierError::NotAFile => {
                write!(f, "Path is not a file")
            }
            ClassifierError::InvalidDateRange => {
                write!(f, "Invalid Date Range.")
            }
            ClassifierError::TimeConversion => {
                write!(f, "Failed to Convert System Time")
            }
        }
    }
}

impl Error for ClassifierError {}
