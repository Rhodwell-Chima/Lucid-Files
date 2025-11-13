pub mod simple;

use std::io::Error;
use std::path::{Path, PathBuf};
use crate::filters::FilterError;

pub trait Scanner {
    fn scan(&self, root: &Path) -> Result<Vec<PathBuf>, FilterError>;
}
