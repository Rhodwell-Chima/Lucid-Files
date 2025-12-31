use crate::classifier::{ClassifierError, FileClassifier};
use log::{debug, error, info, warn};
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
            warn!("Path not found: {}", path.display());
            return Err(ClassifierError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Path does not exist",
            )));
        }

        let ext_os = match path.extension() {
            Some(e) => e,
            None => {
                debug!("Missing extension for path {}", path.display());
                return Err(ClassifierError::MissingExtension);
            }
        };

        let ext_str = match ext_os.to_str() {
            Some(s) => s,
            None => {
                debug!("Invalid extension encoding for path {}", path.display());
                return Err(ClassifierError::InvalidExtensionEncoding);
            }
        };

        let extension = ext_str.to_ascii_lowercase();
        debug!(
            "Extracted extension '{}' from {}",
            extension,
            path.display()
        );

        for (category, extensions) in &self.category_extensions {
            if extensions.contains(&extension) {
                info!("Classified {} as {}", path.display(), category);
                return Ok(Some(category.to_string()));
            }
        }

        debug!(
            "No category matched for extension '{}' (path {})",
            extension,
            path.display()
        );
        Ok(None)
    }
}
