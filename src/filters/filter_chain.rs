use super::{FileFilter, FilterError};
use std::path::Path;

pub struct AndMultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl AndMultiFilter {
    pub fn new(filters: Vec<Box<dyn FileFilter>>) -> Self {
        Self { filters }
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

pub struct OrMultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl OrMultiFilter {
    pub fn new(filters: Vec<Box<dyn FileFilter>>) -> Self {
        Self { filters }
    }
}

impl FileFilter for OrMultiFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        for filter in &self.filters {
            match filter.matches(path)? {
                true => return Ok(true),
                false => continue,
            }
        }
        Ok(false)
    }
}
