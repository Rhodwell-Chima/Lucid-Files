use crate::action::{FileAction, FileActionError};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

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
