use super::{FileFilter, FilterError};
use std::path::Path;

pub struct AndMultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl AndMultiFilter {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn add<F: FileFilter + 'static>(&mut self, filter: F) {
        self.filters.push(Box::new(filter));
    }
}

impl FileFilter for AndMultiFilter {
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
