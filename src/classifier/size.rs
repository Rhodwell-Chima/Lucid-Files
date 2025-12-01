use crate::classifier::{ClassifierError, FileClassifier};
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
        let metadata = metadata(&path).map_err(ClassifierError::Io)?;
        if metadata.is_file() {
            return Err(ClassifierError::NotAFile);
        }
        if self.max_bytes < self.min_bytes {
            return Err(ClassifierError::InvalidRange(
                self.min_bytes,
                self.max_bytes,
            ));
        }
        let len = metadata.len();
        Ok((len >= self.min_bytes && len <= self.max_bytes).then(|| self.category.clone()))
    }
}
