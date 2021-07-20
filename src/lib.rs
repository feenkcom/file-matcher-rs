#[cfg(feature = "regex")]
extern crate regex;

#[cfg(feature = "wildmatch")]
extern crate wildmatch;

mod actions;
mod error;
mod files;
pub(crate) mod finders;
mod utils;
mod alias;

use std::path::PathBuf;
use std::fmt::Debug;

pub use actions::*;

pub use error::{BoxError, Error, Result};
pub use files::{Files, MultipleFiles, OneFile};
pub use alias::FileAlias;

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

pub trait OneFileNamed: Debug {
    fn within_path_buf(&self, directory: PathBuf) -> OneFile;
    fn name_type(&self) -> &FileNamed;
    fn name_alias(&self) -> Option<&str>;
    fn boxed(&self) -> Box<dyn OneFileNamed>;
}

impl OneFileNamed for FileNamed {
    fn within_path_buf(&self, directory: PathBuf) -> OneFile {
        OneFile::new(self.boxed(), directory)
    }

    fn name_type(&self) -> &FileNamed {
        self
    }

    fn name_alias(&self) -> Option<&str> {
        None
    }

    fn boxed(&self) -> Box<dyn OneFileNamed> {
        Box::new(self.clone())
    }
}

impl FileNamed {
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

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneFile {
        self.within_path_buf(directory.into())
    }

    pub fn alias(&self, name: impl Into<String>) -> FileAlias {
        FileAlias::new(self.clone(), name)
    }
}

impl FilesNamed {
    pub fn within(&self, directory: impl Into<PathBuf>) -> MultipleFiles {
        MultipleFiles::new(self.clone(), directory)
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
