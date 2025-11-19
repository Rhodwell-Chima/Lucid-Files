use crate::action::FileAction;
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
    fn execute(&self) -> Result<(), Error> {
        fs::copy(&self.source, &self.destination)?;
        fs::remove_file(&self.source)?;
        Ok(())
    }
}
