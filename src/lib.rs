#[cfg(feature = "regex")]
extern crate regex;

#[cfg(feature = "wildmatch")]
extern crate wildmatch;

mod error;
mod filters;
pub(crate) mod finders;

use std::path::PathBuf;

pub use error::{BoxError, Error};
pub use filters::{MultipleFilesFilter, OneFileFilter};

#[derive(Debug, Clone)]
pub enum FileNamed<'name> {
    Exact(&'name str),
    Any(Vec<&'name str>),
    #[cfg(feature = "regex")]
    Regex(&'name str),
    #[cfg(feature = "wildmatch")]
    Wildmatch(&'name str),
}

#[derive(Debug, Clone)]
pub enum FilesNamed<'name> {
    Exact(&'name str),
    Any(Vec<&'name str>),
    #[cfg(feature = "regex")]
    Regex(&'name str),
    #[cfg(feature = "wildmatch")]
    Wildmatch(&'name str),
}

impl<'name> FileNamed<'name> {
    pub fn within(&self, directory: impl Into<PathBuf>) -> OneFileFilter<'name> {
        OneFileFilter::new(self.clone(), directory)
    }
}

impl<'name> FilesNamed<'name> {
    pub fn within(&self, directory: impl Into<PathBuf>) -> MultipleFilesFilter<'name> {
        MultipleFilesFilter::new(self.clone(), directory)
    }
}
