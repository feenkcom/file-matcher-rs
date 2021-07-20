mod many;
mod one;

pub use many::MultipleFiles;
pub use one::OneFile;

use crate::Result;
use std::path::PathBuf;

pub trait Files {
    fn all(&self) -> Result<Vec<PathBuf>>;
    fn into_files(self) -> Box<dyn Files>;
}
