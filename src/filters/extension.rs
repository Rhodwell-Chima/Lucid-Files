use super::{FileFilter, FilterError};
use std::ffi::OsStr;
use std::path::Path;

#[derive(Clone)]
pub struct ExtensionFilter {
    allowed: Vec<String>,
}

impl ExtensionFilter {
    pub fn new<S: AsRef<str>>(extensions: Vec<S>) -> Self {
        Self {
            allowed: extensions
                .into_iter()
                .map(|s| s.as_ref().to_lowercase())
                .collect(),
        }
    }
}

impl FileFilter for ExtensionFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let result = path
            .extension()
            .and_then(OsStr::to_str)
            .map(|extension| self.allowed.contains(&extension.to_lowercase()));
        Ok(result.unwrap_or_else(|| false))
    }
}
