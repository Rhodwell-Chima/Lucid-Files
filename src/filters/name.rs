use crate::filters::{FileFilter, FilterError};
use log::{debug, info, warn};
use serde::Deserialize;
use std::ffi::OsStr;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub enum NameMatch {
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Equal(String),
}

#[derive(Clone)]
pub struct NameFilter {
    allowed_pattern: NameMatch,
}

impl NameFilter {
    pub fn new(pattern: NameMatch) -> Self {
        Self {
            allowed_pattern: pattern,
        }
    }
}

impl FileFilter for NameFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        debug!("Evaluating NameFilter for path: {}", path.display());

        let name_str = match path.file_name().and_then(OsStr::to_str) {
            Some(n) => n,
            None => {
                warn!("Unable to extract file name from path: {}", path.display());
                return Err(FilterError::Other("Unable to Get File name.".to_string()));
            }
        };

        let result = match &self.allowed_pattern {
            NameMatch::Contains(pattern) => {
                let matched = name_str.contains(pattern);
                debug!(
                    "NameMatch::Contains: name=`{}`, pattern=`{}`, matched={}",
                    name_str, pattern, matched
                );
                matched
            }
            NameMatch::StartsWith(pattern) => {
                let matched = name_str.starts_with(pattern);
                debug!(
                    "NameMatch::StartsWith: name=`{}`, pattern=`{}`, matched={}",
                    name_str, pattern, matched
                );
                matched
            }
            NameMatch::EndsWith(pattern) => {
                let matched = name_str.ends_with(pattern);
                debug!(
                    "NameMatch::EndsWith: name=`{}`, pattern=`{}`, matched={}",
                    name_str, pattern, matched
                );
                matched
            }
            NameMatch::Equal(pattern) => {
                let matched = name_str == pattern;
                debug!(
                    "NameMatch::Equal: name=`{}`, pattern=`{}`, matched={}",
                    name_str, pattern, matched
                );
                matched
            }
        };

        info!("NameFilter result for `{}` -> {}", name_str, result);
        Ok(result)
    }
}
