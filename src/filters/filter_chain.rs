use super::{FileFilter, FilterError};
use std::path::Path;

pub struct MultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl MultiFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add(mut self, filter: Box<dyn FileFilter>) -> Self {
        (&mut self.filters).push(filter);
        self
    }
}

impl FileFilter for MultiFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        for filter in &self.filters {
            match filter.matches(path)? {
                true => continue,
                false => return Ok(false),
            }
        }

        Ok(true)
    }
}
