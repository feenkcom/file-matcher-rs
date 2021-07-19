#[cfg(feature = "regex")]
extern crate regex;

#[cfg(feature = "wildmatch")]
extern crate wildmatch;

mod error;
mod filters;
pub(crate) mod finders;
mod utils;

use std::path::PathBuf;

pub use error::{BoxError, Error, Result};
pub use filters::{FilesFilter, MultipleFilesFilter, OneFileFilter};

/// Defines a file with various name types. It is required that exactly one file with a given name description exists.
#[derive(Debug, Clone)]
pub enum FileNamed {
    Exact(String),
    Any(Vec<String>),
    #[cfg(feature = "regex")]
    Regex(String),
    #[cfg(feature = "wildmatch")]
    Wildmatch(String),
}

#[derive(Debug, Clone)]
pub enum FilesNamed {
    Exact(String),
    Any(Vec<String>),
    #[cfg(feature = "regex")]
    Regex(String),
    #[cfg(feature = "wildmatch")]
    Wildmatch(String),
}

impl FileNamed {
    pub fn within(&self, directory: impl Into<PathBuf>) -> OneFileFilter {
        OneFileFilter::new(self.clone(), directory)
    }

    pub fn exact(name: impl Into<String>) -> Self {
        Self::Exact(name.into())
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self::Any(names.into_iter().map(|name| name.into()).collect())
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self::Regex(pattern.into())
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self::Wildmatch(pattern.into())
    }
}

impl FilesNamed {
    pub fn within(&self, directory: impl Into<PathBuf>) -> MultipleFilesFilter {
        MultipleFilesFilter::new(self.clone(), directory)
    }

    pub fn exact(name: impl Into<String>) -> Self {
        Self::Exact(name.into())
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self::Any(names.into_iter().map(|name| name.into()).collect())
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self::Regex(pattern.into())
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self::Wildmatch(pattern.into())
    }
}
