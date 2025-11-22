use crate::action::{FileAction, FileActionError};
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct CopyAction {
    source: PathBuf,
    destination: PathBuf,
}

impl CopyAction {
    pub fn new(source: impl Into<PathBuf>, destination: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }
}

impl FileAction for CopyAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}

pub struct CopyActionRef<'a> {
    source: &'a Path,
    destination: &'a Path,
}

impl<'a> CopyActionRef<'a> {
    pub fn new(source: &'a Path, destination: &'a Path) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl<'a> FileAction for CopyActionRef<'a> {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(self.source, self.destination).map_err(FileActionError::Io)?;
        Ok(())
    }
}
