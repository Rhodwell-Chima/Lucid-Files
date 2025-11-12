use super::{FileFilter, FilterError};
use std::fs::metadata;
use std::path::Path;

pub struct SizeFilter {
    min_bytes: u64,
    max_bytes: u64,
}

impl SizeFilter {
    pub fn new(min: u64, max: u64) -> Self {
        Self {
            min_bytes: min,
            max_bytes: max,
        }
    }
}

impl FileFilter for SizeFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let len = metadata(&path).map_err(FilterError::IoError)?.len();
        Ok(len >= self.min_bytes && len <= self.max_bytes)
    }
}
