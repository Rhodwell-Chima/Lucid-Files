use crate::filters::{FileFilter, FilterError};
use bitflags::bitflags;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct PermissionMask: u8 {
        const READ    = 0b001;
        const WRITE   = 0b010;
        const EXECUTE = 0b100;

        const READ_WRITE    = Self::READ.bits() | Self::WRITE.bits();
        const READ_EXEC     = Self::READ.bits() | Self::EXECUTE.bits();
        const WRITE_EXEC    = Self::WRITE.bits() | Self::EXECUTE.bits();
        const ALL           = Self::READ.bits() | Self::WRITE.bits() | Self::EXECUTE.bits();
    }
}

pub struct PermissionFilter {
    permission: PermissionMask,
}

impl PermissionFilter {
    pub fn new(permission: PermissionMask) -> Self {
        Self { permission }
    }
}

impl FileFilter for PermissionFilter {
    fn matches(&self, path: &Path) -> Result<bool, FilterError> {
        let metadata = fs::metadata(path).map_err(FilterError::IoError)?;
        let file_permissions = permissions_from_metadata(&metadata);
        Ok(file_permissions.contains(self.permission))
    }
}

fn permissions_from_metadata(metadata: &fs::Metadata) -> PermissionMask {
    let mode = metadata.permissions().mode();

    let mut mask = PermissionMask::empty();

    if mode & 0o444 != 0 {
        mask |= PermissionMask::READ;
    }
    if mode & 0o222 != 0 {
        mask |= PermissionMask::WRITE;
    }
    if mode & 0o111 != 0 {
        mask |= PermissionMask::EXECUTE;
    }

    mask
}
