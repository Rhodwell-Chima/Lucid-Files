use crate::action::{FileAction, FileActionError};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct MoveAction {
    source: PathBuf,
    destination: PathBuf,
}
impl MoveAction {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl FileAction for MoveAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::rename(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}
