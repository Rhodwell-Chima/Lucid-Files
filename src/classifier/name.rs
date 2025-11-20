use crate::classifier::{ClassifierError, FileClassifier};
use crate::filters::name::NameMatch;
use std::ffi::OsStr;
use std::path::Path;

pub struct NameClassifier {
    category: String,
    allowed_pattern: NameMatch,
}

impl NameClassifier {
    pub fn new(category: impl Into<String>, allowed_pattern: NameMatch) -> Self {
        Self {
            category: category.into(),
            allowed_pattern,
        }
    }
}

impl FileClassifier for NameClassifier {
    fn classify(&self, path: &Path) -> Result<Option<String>, ClassifierError> {
        let name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(ClassifierError::InvalidFileName)?;

        let matched = match &self.allowed_pattern {
            NameMatch::Contains(pattern) => name.contains(pattern),
            NameMatch::StartsWith(pattern) => name.starts_with(pattern),
            NameMatch::EndsWith(pattern) => name.ends_with(pattern),
            NameMatch::Equal(pattern) => name.eq(pattern),
        };
        Ok(matched.then(|| self.category.clone()))
    }
}
