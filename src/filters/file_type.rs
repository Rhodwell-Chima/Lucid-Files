use super::{FileFilter, FilterError};
use std::fs::symlink_metadata;
use std::path::Path;

#[derive(Clone)]
pub enum FileKind {
    Symlink,
    Directory,
    File,
}

#[derive(Clone)]
pub struct FileTypeFilter {
    kind: FileKind,
}

impl FileTypeFilter {
    pub fn new(kind: FileKind) -> Self {
        Self { kind }
    }
}

impl FileFilter for FileTypeFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let metadata = symlink_metadata(&path).map_err(FilterError::IoError)?;
        Ok(match self.kind {
            FileKind::Symlink => metadata.file_type().is_symlink(),
            FileKind::Directory => metadata.is_dir(),
            FileKind::File => metadata.is_file(),
        })
    }
}
