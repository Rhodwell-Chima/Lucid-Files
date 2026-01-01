use super::{FileFilter, FilterError};
use log::{debug, info, warn};
use std::fs::metadata;
use std::path::Path;

#[derive(Clone)]
pub struct SizeFilter {
    min_bytes: u64,
    max_bytes: u64,
}

impl SizeFilter {
    pub fn new(min: u64, max: u64) -> Self {
        info!("SizeFilter created with min={} max={}", min, max);
        Self {
            min_bytes: min,
            max_bytes: max,
        }
    }
}

impl FileFilter for SizeFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        debug!(
            "SizeFilter: evaluating {:?} (min={}, max={})",
            path, self.min_bytes, self.max_bytes
        );

        match metadata(path) {
            Ok(md) => {
                let len = md.len();
                let matched = len >= self.min_bytes && len <= self.max_bytes;
                info!(
                    "SizeFilter: path={:?} len={} min={} max={} -> {}",
                    path, len, self.min_bytes, self.max_bytes, matched
                );
                Ok(matched)
            }
            Err(e) => {
                warn!(
                    "SizeFilter: failed to get metadata for {:?} -> {:?}",
                    path, e
                );
                Err(FilterError::IoError(e))
            }
        }
    }
}
