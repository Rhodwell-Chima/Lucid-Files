use crate::filters::{FileFilter, FilterError};
use crate::scanner::Scanner;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct RecursiveScanner {
    filter: Box<dyn FileFilter>,
    min_depth: usize,
    max_depth: usize,
}

impl RecursiveScanner {
    pub fn new(filter: Box<dyn FileFilter>, min_depth: usize, max_depth: usize) -> Self {
        Self {
            filter,
            min_depth,
            max_depth,
        }
    }

    pub fn set_min_depth(&mut self, min_depth: usize) {
        self.min_depth = min_depth;
    }

    pub fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }
}

impl Scanner for RecursiveScanner {
    fn scan(&self, root: impl AsRef<Path>) -> Result<Vec<PathBuf>, FilterError> {
        let mut paths = vec![];
        for entry in WalkDir::new(root.as_ref())
            .follow_root_links(false)
            .min_depth(self.min_depth)
            .max_depth(self.max_depth)
        {
            let file_path = entry.map_err(FilterError::WalkdirError)?.into_path();
            if self.filter.matches(&file_path)? {
                paths.push(file_path);
            }
        }
        Ok(paths)
    }
}
