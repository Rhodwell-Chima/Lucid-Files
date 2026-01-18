use std::fs;
use std::io;
use std::path::Path;

pub fn create_directory_with_validation<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    if path.exists() {
        return if path.is_dir() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Path exists but is not a directory",
            ))
        };
    }

    if let Some(parent) = path.parent() {
        if !parent.exists() {
        } else if !parent.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Parent path exists but is not a directory",
            ));
        }
    }

    if path.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path cannot be empty",
        ));
    }

    fs::create_dir_all(path)?;

    if path.is_dir() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Directory creation succeeded but verification failed",
        ))
    }
}
