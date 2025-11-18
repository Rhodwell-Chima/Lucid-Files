pub mod extension;
mod date;
mod size;
mod name;

pub use extension::*;

use std::io::Error;
use std::path::Path;

pub trait FileClassifier {
    fn classify(&self, path: &Path) -> Result<String, Error>;
}
