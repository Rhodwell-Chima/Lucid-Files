use std::path::Path;

mod date;
mod extension;
mod file_type;
mod name;
mod owner;
mod permissions;
mod size;

pub trait FileFilter {
    fn matches(&self, path: &Path) -> bool;
}
