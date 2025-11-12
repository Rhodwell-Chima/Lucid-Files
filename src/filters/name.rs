use crate::filters::{FileFilter, FilterError};
use std::ffi::OsStr;
use std::path::Path;

pub enum NameMatch {
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Equal(String),
}

pub struct NameFilter {
    allowed_pattern: NameMatch,
}

impl NameFilter {
    fn new(pattern: NameMatch) -> Self {
        Self {
            allowed_pattern: pattern,
        }
    }
}

impl FileFilter for NameFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FilterError::Other("Unable to Get File name.".to_string()));

        Ok(match &self.allowed_pattern {
            NameMatch::Contains(pattern) => name?.contains(pattern),
            NameMatch::StartsWith(pattern) => name?.starts_with(pattern),
            NameMatch::EndsWith(pattern) => name?.ends_with(pattern),
            NameMatch::Equal(pattern) => name?.eq(pattern),
        })
    }
}
