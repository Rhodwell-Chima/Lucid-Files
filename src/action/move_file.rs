use crate::action::{FileAction, FileActionError};
use std::fs;
use std::path::{Path, PathBuf};

pub struct MoveAction {
    source: PathBuf,
    destination: PathBuf,
}
impl MoveAction {
    pub fn new(source: impl Into<PathBuf>, destination: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }
}

impl FileAction for MoveAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::rename(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}

pub struct MoveActionRef<'a> {
    source: &'a Path,
    destination: &'a Path,
}

impl<'a> MoveActionRef<'a> {
    pub fn new(source: &'a Path, destination: &'a Path) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl<'a> FileAction for MoveActionRef<'a> {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::rename(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}
