mod date;
pub mod extension;
mod name;
mod size;

pub use extension::*;
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
        }
    }
}

impl Error for ClassifierError {}
