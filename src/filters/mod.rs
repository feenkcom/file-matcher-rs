mod many;
mod one;

pub use many::MultipleFilesFilter;
pub use one::OneFileFilter;

use crate::Result;
use std::path::PathBuf;

pub trait FilesFilter {
    fn all(&self) -> Result<Vec<PathBuf>>;
    fn into_filter(self) -> Box<dyn FilesFilter>;
}
