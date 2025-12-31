pub mod recursive;
pub mod simple;

pub use recursive::*;
pub use simple::*;

use crate::filters::FilterError;
use std::path::{Path, PathBuf};

pub trait Scanner {
    fn scan(&self, root: impl AsRef<Path>) -> Result<Vec<PathBuf>, FilterError>;
}
