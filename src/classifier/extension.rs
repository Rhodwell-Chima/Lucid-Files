use crate::classifier::{ClassifierError, FileClassifier};
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
    fn classify(&self, path: &Path) -> Result<Option<String>, ClassifierError> {
        let extension = path
            .extension()
            .ok_or(ClassifierError::MissingExtension)?
            .to_str()
            .ok_or(ClassifierError::InvalidExtensionEncoding)?
            .to_lowercase();
        for (category, extensions) in &self.category_extensions {
            if extensions.contains(&extension) {
                return Ok(Some(category.to_string()));
            }
        }
        Ok(None)
    }
}
