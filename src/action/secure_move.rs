use crate::action::{FileAction, FileActionError};
use std::fs;
use std::path::{Path, PathBuf};

pub struct SecureMoveAction {
    source: PathBuf,
    destination: PathBuf,
}
impl SecureMoveAction {
    pub fn new(source: impl Into<PathBuf>, destination: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }
}

impl FileAction for SecureMoveAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        fs::remove_file(&self.source).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}

pub struct SecureMoveActionRef<'a> {
    source: &'a Path,
    destination: &'a Path,
}

impl<'a> SecureMoveActionRef<'a> {
    pub fn new(source: &'a Path, destination: &'a Path) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl<'a> FileAction for SecureMoveActionRef<'a> {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        fs::remove_file(&self.source).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}
