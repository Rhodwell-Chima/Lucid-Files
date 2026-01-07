use crate::action::{FileAction, FileActionError};
use std::fs;
use std::path::{Path, PathBuf};

pub struct CopyAction {
    source: PathBuf,
    destination: PathBuf,
}

impl CopyAction {
    pub fn new(source: impl Into<PathBuf>, destination: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
        }
    }
}

impl FileAction for CopyAction {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(&self.source, &self.destination).map_err(|e| FileActionError::Io(e))?;
        Ok(())
    }
}

pub struct CopyActionRef<'a> {
    source: &'a Path,
    destination: &'a Path,
}

impl<'a> CopyActionRef<'a> {
    pub fn new(source: &'a Path, destination: &'a Path) -> Self {
        Self {
            source,
            destination,
        }
    }
}

impl<'a> FileAction for CopyActionRef<'a> {
    fn execute(&self) -> Result<(), FileActionError> {
        fs::copy(self.source, self.destination).map_err(FileActionError::Io)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn copy_ref_fail_test() {
        let copy_struct = CopyActionRef {
            source: Path::new(""),
            destination: Path::new(""),
        };

        assert!(copy_struct.execute().is_err())
    }

    #[test]
    fn copy_ref_success_test() {
        let mut src_file = NamedTempFile::new().unwrap();
        writeln!(src_file, "Source").unwrap();
        let src_path = src_file.path();

        let dest_file = NamedTempFile::new().unwrap();
        let dest_path = dest_file.path().to_path_buf();
        drop(dest_file);

        let copy_action = CopyActionRef::new(src_path, &dest_path);
        assert!(copy_action.execute().is_ok());

        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content.trim(), "Source");
    }

    #[test]
    fn copy_fail_test() {
        let copy_struct = CopyAction {
            source: PathBuf::from(""),
            destination: PathBuf::from(""),
        };

        assert!(copy_struct.execute().is_err())
    }

    #[test]
    fn copy_success_test() {
        let mut src_file = NamedTempFile::new().unwrap();
        writeln!(src_file, "Source").unwrap();
        let src_path = src_file.path().to_path_buf();

        let dest_file = NamedTempFile::new().unwrap();
        let dest_path = dest_file.path().to_path_buf();
        drop(dest_file);

        let copy_action = CopyAction::new(src_path, dest_path.clone());
        assert!(copy_action.execute().is_ok());

        let content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(content.trim(), "Source");
    }
}
