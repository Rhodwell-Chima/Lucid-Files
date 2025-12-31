use crate::classifier::{ClassifierError, FileClassifier};
use log::{debug, info, warn};
use std::fs::metadata;
use std::path::Path;

pub struct SizeClassifier {
    category: String,
    min_bytes: u64,
    max_bytes: u64,
}

impl SizeClassifier {
    pub fn new(category: impl Into<String>, min_bytes: u64, max_bytes: u64) -> Self {
        if min_bytes > max_bytes {
            return Self {
                category: category.into(),
                min_bytes: max_bytes,
                max_bytes: min_bytes,
            };
        }
        Self {
            category: category.into(),
            min_bytes,
            max_bytes,
        }
    }
}

impl FileClassifier for SizeClassifier {
    fn classify(&self, path: &Path) -> Result<Option<String>, ClassifierError> {
        if self.max_bytes < self.min_bytes {
            warn!(
                "Invalid byte range for category '{}' (min={} max={})",
                self.category, self.min_bytes, self.max_bytes
            );
            return Err(ClassifierError::InvalidRange(
                self.min_bytes,
                self.max_bytes,
            ));
        }

        let metadata = metadata(path).map_err(ClassifierError::Io)?;
        if !metadata.is_file() {
            warn!("Path is not a file: {}", path.display());
            return Err(ClassifierError::NotAFile);
        }

        let len = metadata.len();
        debug!(
            "File {} size = {} bytes (range {}..={})",
            path.display(),
            len,
            self.min_bytes,
            self.max_bytes
        );

        if (len >= self.min_bytes) && (len <= self.max_bytes) {
            info!("Classified {} as {}", path.display(), self.category);
            Ok(Some(self.category.clone()))
        } else {
            debug!(
                "File {} did not match size range ({}..={})",
                path.display(),
                self.min_bytes,
                self.max_bytes
            );
            Ok(None)
        }
    }
}
