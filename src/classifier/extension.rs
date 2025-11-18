use crate::classifier::FileClassifier;
use ErrorKind::Other;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct ExtensionClassifier {
    category_extensions: HashMap<String, Vec<String>>,
}

impl ExtensionClassifier {
    pub fn new(category_extensions: HashMap<String, Vec<String>>) -> Self {
        Self {
            category_extensions,
        }
    }
}

impl FileClassifier for ExtensionClassifier {
    fn classify(&self, path: &Path) -> Result<String, Error> {
        let extension = path
            .extension()
            .ok_or_else(|| Other)?
            .to_str()
            .unwrap_or_else(|| "Unable to convert &OsStr to &str")
            .to_lowercase();
        for (category, extensions) in &self.category_extensions {
            if extensions.contains(&extension) {
                return Ok(category.to_string());
            }
        }

        Ok("Unknown".to_string())
    }
}
