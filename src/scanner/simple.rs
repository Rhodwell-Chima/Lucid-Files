use crate::filters::filter_chain::MultiFilter;
use crate::filters::{FileFilter, FilterError};
use crate::scanner::Scanner;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SimpleScanner {
    filter: MultiFilter,
}

impl SimpleScanner {
    pub fn new(filter: MultiFilter) -> Self {
        Self { filter }
    }
}

impl Scanner for SimpleScanner {
    fn scan(&self, root: &Path) -> Result<Vec<PathBuf>, FilterError> {
        let mut paths = vec![];
        for entry in fs::read_dir(root).map_err(|e| FilterError::IoError(e))? {
            let file_path = entry.map_err(|e| FilterError::IoError(e))?.path();
            if self.filter.matches(&file_path)? {
                paths.push(file_path);
            }
        }
        Ok(paths)
    }
}
