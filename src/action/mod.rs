use std::io::Error;
use std::path::Path;

pub mod copy;
pub mod delete;
pub mod move_file;

pub use copy::*;
pub use delete::*;
pub use move_file::*;

pub trait FileAction {
    fn execute(&self) -> Result<(), Error>;
}
