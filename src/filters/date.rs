use super::{FileFilter, FilterError};
use std::fs::metadata;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub enum DateType {
    Modified,
    Accessed,
    Created,
}

pub struct DateFilter {
    date_type: DateType,
    within: Duration,
}

impl DateFilter {
    pub fn new(date_type: DateType, duration: Duration) -> Self {
        Self {
            date_type,
            within: duration,
        }
    }
}

impl FileFilter for DateFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let metadata = metadata(&path).map_err(FilterError::IoError)?;
        let system_time = match self.date_type {
            DateType::Modified => metadata.modified().map_err(FilterError::IoError)?,
            DateType::Accessed => metadata.accessed().map_err(FilterError::IoError)?,
            DateType::Created => metadata.created().map_err(FilterError::IoError)?,
        };
        let elapsed_time = SystemTime::now()
            .duration_since(system_time)
            .map_err(FilterError::TimeError)?;

        Ok(elapsed_time <= self.within)
    }
}
