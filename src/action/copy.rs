use crate::action::FileAction;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct CopyAction {
    source: PathBuf,
    destination: PathBuf,
}

impl CopyAction {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl FileAction for CopyAction {
    fn execute(&self) -> Result<(), Error> {
        fs::copy(&self.source, &self.destination)?;
        Ok(())
    }
}
