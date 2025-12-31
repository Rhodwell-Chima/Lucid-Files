use super::{FileFilter, FilterError};
use log::{debug, info, warn};
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
        match path.extension().and_then(OsStr::to_str) {
            Some(ext) => {
                let ext_lower = ext.to_lowercase();
                debug!(
                    "Extracted extension '{}' from {}",
                    ext_lower,
                    path.display()
                );
                let matched = self.allowed.contains(&ext_lower);
                if matched {
                    info!(
                        "Path {} matched allowed extension '{}'",
                        path.display(),
                        ext_lower
                    );
                } else {
                    debug!(
                        "Path {} extension '{}' not in allowed list",
                        path.display(),
                        ext_lower
                    );
                }
                Ok(matched)
            }
            None => {
                warn!("Missing or non-UTF8 extension for path {}", path.display());
                Ok(false)
            }
        }
    }
}
