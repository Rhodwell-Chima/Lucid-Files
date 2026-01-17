use crate::config::ScannerType;
use crate::filters::{FileFilter, FilterError};
use crate::scanner::{RecursiveScanner, Scanner, SimpleScanner};
use std::path::{Path, PathBuf};

pub fn perform_scanning(
    choice: &ScannerType,
    source: &Path,
    filter: Box<dyn FileFilter>,
) -> Result<Vec<PathBuf>, FilterError> {
    match choice {
        ScannerType::Simple => SimpleScanner::new(filter).scan(source),
        ScannerType::Recursive {
            min_depth,
            max_depth,
        } => RecursiveScanner::new(filter, *min_depth, *max_depth).scan(source),
        ScannerType::Unknown => Err(FilterError::Other("The Scanner is Unknown".to_string())),
    }
}
