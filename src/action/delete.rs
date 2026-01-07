use crate::action::{FileAction, FileActionError};
use std::fs;
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn delete_success_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path();
        assert!(&temp_path.exists());

        let delete_action = DeleteAction::new(&temp_path);
        delete_action.execute().expect("Its suppose to never Fail.");

        assert!(!&temp_path.exists())
    }

    #[test]
    fn delete_fail_test() {
        let temp_file = TempDir::new().unwrap();
        let temp_path = temp_file.path();
        assert!(&temp_path.exists());
        assert!(&temp_path.is_dir());

        let delete_action = DeleteAction::new(&temp_path).execute();
        assert!(delete_action.is_err());
        assert!(&temp_path.exists())
    }

    #[test]
    fn delete_ref_success_test() {
        let temp_file = NamedTempFile::new().unwrap();
        let temp_path = temp_file.path();
        assert!(&temp_path.exists());

        let delete_action = DeleteActionRef::new(&temp_path);
        delete_action.execute().expect("Its suppose to never Fail.");

        assert!(!&temp_path.exists())
    }

    #[test]
    fn delete_ref_fail_test() {
        let temp_file = TempDir::new().unwrap();
        let temp_path = temp_file.path();
        assert!(&temp_path.exists());
        assert!(&temp_path.is_dir());

        let delete_action = DeleteActionRef::new(&temp_path).execute();
        assert!(delete_action.is_err());
        assert!(&temp_path.exists())
    }
}
