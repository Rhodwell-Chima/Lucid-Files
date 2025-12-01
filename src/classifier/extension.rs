use crate::classifier::{ClassifierError, FileClassifier};
use std::collections::HashMap;
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
        if !path.try_exists().map_err(ClassifierError::Io)? {
            return Err(ClassifierError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist",
            )));
        }
        let extension = path
            .extension()
            .ok_or(ClassifierError::MissingExtension)?
            .to_str()
            .ok_or(ClassifierError::InvalidExtensionEncoding)?
            .to_ascii_lowercase();
        for (category, extensions) in &self.category_extensions {
            if extensions.contains(&extension) {
                return Ok(Some(category.to_string()));
            }
        }
        Ok(None)
    }
}
