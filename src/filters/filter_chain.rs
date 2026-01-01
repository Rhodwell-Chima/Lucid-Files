use super::{FileFilter, FilterError};
use log::{debug, info, warn};
use std::path::Path;

pub struct AndMultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl AndMultiFilter {
    pub fn new(filters: Vec<Box<dyn FileFilter>>) -> Self {
        let f = Self { filters };
        info!("AndMultiFilter created with {} filter(s)", f.filters.len());
        f
    }

    pub fn add<F: FileFilter + 'static>(&mut self, filter: F) {
        let name = std::any::type_name_of_val(&filter);
        self.filters.push(Box::new(filter));
        info!("AndMultiFilter: added filter {}", name);
    }
}

impl FileFilter for AndMultiFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        debug!(
            "AndMultiFilter: evaluating {} filter(s) for {:?}",
            self.filters.len(),
            path
        );
        for (i, filter) in self.filters.iter().enumerate() {
            let fname = std::any::type_name_of_val(&**filter);
            debug!(
                "AndMultiFilter: evaluating filter #{} ({}) for {:?}",
                i, fname, path
            );

            match filter.matches(path) {
                Ok(true) => {
                    info!("AndMultiFilter: filter #{} passed", i);
                    continue;
                }
                Ok(false) => {
                    info!("AndMultiFilter: filter #{} failed -> returning false", i);
                    return Ok(false);
                }
                Err(e) => {
                    warn!(
                        "AndMultiFilter: filter #{} returned error -> propagating: {:?}",
                        i, e
                    );
                    return Err(e);
                }
            }
        }

        info!("AndMultiFilter: all filters passed -> returning true");
        Ok(true)
    }
}

pub struct OrMultiFilter {
    filters: Vec<Box<dyn FileFilter>>,
}

impl OrMultiFilter {
    pub fn new(filters: Vec<Box<dyn FileFilter>>) -> Self {
        let f = Self { filters };
        info!("OrMultiFilter created with {} filter(s)", f.filters.len());
        f
    }
}

impl FileFilter for OrMultiFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        debug!(
            "OrMultiFilter: evaluating {} filter(s) for {:?}",
            self.filters.len(),
            path
        );
        for (i, filter) in self.filters.iter().enumerate() {
            let fname = std::any::type_name_of_val(&**filter);
            debug!(
                "OrMultiFilter: evaluating filter #{} ({}) for {:?}",
                i, fname, path
            );

            match filter.matches(path) {
                Ok(true) => {
                    info!("OrMultiFilter: filter #{} passed -> returning true", i);
                    return Ok(true);
                }
                Ok(false) => {
                    info!("OrMultiFilter: filter #{} did not match, continuing", i);
                    continue;
                }
                Err(e) => {
                    warn!(
                        "OrMultiFilter: filter #{} returned error -> propagating: {:?}",
                        i, e
                    );
                    return Err(e);
                }
            }
        }
        info!("OrMultiFilter: no filters matched -> returning false");
        Ok(false)
    }
}

pub struct NotGateFilter {
    filters: Box<dyn FileFilter>,
}

impl NotGateFilter {
    pub fn new(filters: Box<dyn FileFilter>) -> Self {
        info!("NotGateFilter created");
        Self { filters }
    }
}

impl FileFilter for NotGateFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        debug!("NotGateFilter: evaluating inner filter for {:?}", path);
        match self.filters.matches(path) {
            Ok(res) => {
                info!(
                    "NotGateFilter: inner result = {} -> returning {}",
                    res, !res
                );
                Ok(!res)
            }
            Err(e) => {
                warn!(
                    "NotGateFilter: inner filter returned error -> propagating: {:?}",
                    e
                );
                Err(e)
            }
        }
    }
}
