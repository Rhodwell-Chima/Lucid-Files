use crate::action::FileAction;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

pub struct DeleteAction {
    path: PathBuf,
}

impl DeleteAction {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl FileAction for DeleteAction {
    fn execute(&self) -> Result<(), Error> {
        fs::remove_file(&self.path)?;
        Ok(())
    }
}
