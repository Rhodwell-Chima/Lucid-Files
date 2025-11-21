use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod copy;
pub mod delete;
pub mod move_file;
pub mod secure_move;

pub use copy::*;
pub use delete::*;

pub trait FileAction {
    fn execute(&self) -> Result<(), FileActionError>;
}

#[derive(Debug)]
pub enum FileActionError {
    Io(std::io::Error),
}
impl Display for FileActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FileActionError::Io(e) => {
                write!(f, "File Action IO Error: {}", e)
            }
        }
    }
}
impl Error for FileActionError {}
