use super::{FileFilter, FilterError};
use log::{debug, info, warn};
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
        let metadata = match symlink_metadata(path) {
            Ok(m) => {
                debug!("Obtained metadata for {}", path.display());
                m
            }
            Err(e) => {
                warn!("Failed to read metadata for {}: {}", path.display(), e);
                return Err(FilterError::IoError(e));
            }
        };

        let kind_name = match self.kind {
            FileKind::Symlink => "symlink",
            FileKind::Directory => "directory",
            FileKind::File => "file",
        };

        let matched = match self.kind {
            FileKind::Symlink => metadata.file_type().is_symlink(),
            FileKind::Directory => metadata.is_dir(),
            FileKind::File => metadata.is_file(),
        };

        if matched {
            info!("Path {} is a {}", path.display(), kind_name);
        } else {
            debug!("Path {} is not a {}", path.display(), kind_name);
        }

        Ok(matched)
    }
}
