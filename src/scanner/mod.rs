pub mod simple;

use std::path::{Path, PathBuf};

pub trait Scanner {
    fn scan(&self, root: &Path) -> Vec<PathBuf>;
}
