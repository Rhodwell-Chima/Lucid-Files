use crate::action::{FileAction, FileActionError};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct SecureMoveAction {
    source: PathBuf,
    destination: PathBuf,
}
impl SecureMoveAction {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        Self {
            source,
            destination,
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
