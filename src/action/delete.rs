use crate::action::{FileAction, FileActionError};
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct DeleteAction {
    path: PathBuf,
}

impl DeleteAction {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl FileAction for DeleteAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::remove_file(&self.path).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}

pub struct DeleteActionRef<'a> {
    path: &'a Path,
}

impl<'a> DeleteActionRef<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl<'a> FileAction for DeleteActionRef<'a> {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::remove_file(&self.path).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}
