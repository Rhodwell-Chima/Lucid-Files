use crate::filters::{FileFilter, FilterError};
use crate::scanner::Scanner;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SimpleScanner {
    filter: Box<dyn FileFilter>,
}

impl SimpleScanner {
    pub fn new(filter: Box<dyn FileFilter>) -> Self {
        Self { filter }
    }
}

impl Scanner for SimpleScanner {
    fn scan(&self, root: impl AsRef<Path>) -> Result<Vec<PathBuf>, FilterError> {
        let mut paths = vec![];
        for entry in fs::read_dir(root.as_ref()).map_err(|e| FilterError::IoError(e))? {
            let file_path = entry.map_err(|e| FilterError::IoError(e))?.path();
            if self.filter.matches(&file_path)? {
                paths.push(file_path);
            }
        }
        Ok(paths)
    }
}
