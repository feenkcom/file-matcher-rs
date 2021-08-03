#[cfg(feature = "fs_extra")]
extern crate fs_extra;
#[cfg(feature = "regex")]
extern crate regex;
#[cfg(feature = "wildmatch")]
extern crate wildmatch;

mod actions;
mod alias;
mod entries;
mod error;
mod finders;
mod utils;

use std::fmt::Debug;
use std::path::PathBuf;

pub use actions::*;
pub use entries::*;

pub use alias::EntityAlias;
pub use error::{FileMatcherError, Result};

/// Defines a file with various name types.
/// It is required that there exists exactly one file with a given name description.
/// ```
/// use file_matcher::FileNamed;
///
/// # use file_matcher::Result;
/// # fn main() -> Result<()> {
///     // Find an exactly one file matching cat* within tests/assets folder
///     FileNamed::wildmatch("cat*")
///         .within("tests/assets")
///         .find()?;
///
///     // Check there exists a file matching cat* within tests/assets folder
///     FileNamed::wildmatch("cat*")
///         .within("tests/assets")
///         .exists()?;
/// #    Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FileNamed(EntryNamed);

/// Defines a folder with various name types.
/// It is required that there exists exactly one folder with a given name description.
/// ```
/// use file_matcher::FolderNamed;
///
/// # use file_matcher::Result;
/// # fn main() -> Result<()> {
///     // Find an exactly one folder matching cat* within tests/assets folder
///     FolderNamed::wildmatch("cat*")
///         .within("tests/assets")
///         .find()?;
///
///     // Check there exists a folder matching cat* within tests/assets folder
///     FolderNamed::wildmatch("cat*")
///         .within("tests/assets")
///         .exists()?;
/// #    Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct FolderNamed(EntryNamed);

/// Defines a file or a folder with various name types.
/// It is required that there exists exactly one file or folder with a given name description.
#[derive(Debug, Clone)]
pub struct FileOrFolderNamed(EntryNamed);

#[derive(Debug, Clone)]
pub struct FilesNamed(EntryNamed);

impl OneEntryNamed for FileNamed {
    fn within_path_buf(&self, directory: PathBuf) -> OneEntry {
        OneEntry::new(self.boxed(), directory)
    }

    fn entry_name(&self) -> &EntryName {
        self.0.entry_name()
    }

    fn entry_type(&self) -> &EntryType {
        self.0.entry_type()
    }

    fn name_alias(&self) -> Option<&str> {
        None
    }

    fn boxed(&self) -> Box<dyn OneEntryNamed> {
        Box::new(self.clone())
    }
}

impl FileNamed {
    pub fn exact(name: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Exact(name.into())))
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self(EntryNamed::file(EntryName::Any(
            names.into_iter().map(|name| name.into()).collect(),
        )))
    }

    pub fn any_named(names: Vec<FileNamed>) -> Self {
        Self(EntryNamed::file(EntryName::AnyNamed(
            names
                .into_iter()
                .map(|name| name.entry_name().clone())
                .collect(),
        )))
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Regex(pattern.into())))
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Wildmatch(pattern.into())))
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneEntry {
        self.within_path_buf(directory.into())
    }

    pub fn alias(&self, name: impl Into<String>) -> EntityAlias {
        EntityAlias::new(Box::new(self.clone()), name)
    }
}

impl OneEntryNamed for FolderNamed {
    fn within_path_buf(&self, directory: PathBuf) -> OneEntry {
        OneEntry::new(self.boxed(), directory)
    }

    fn entry_name(&self) -> &EntryName {
        self.0.entry_name()
    }

    fn entry_type(&self) -> &EntryType {
        self.0.entry_type()
    }

    fn name_alias(&self) -> Option<&str> {
        None
    }

    fn boxed(&self) -> Box<dyn OneEntryNamed> {
        Box::new(self.clone())
    }
}

impl FolderNamed {
    pub fn exact(name: impl Into<String>) -> Self {
        Self(EntryNamed::folder(EntryName::Exact(name.into())))
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self(EntryNamed::folder(EntryName::Any(
            names.into_iter().map(|name| name.into()).collect(),
        )))
    }

    pub fn any_named(names: Vec<FolderNamed>) -> Self {
        Self(EntryNamed::folder(EntryName::AnyNamed(
            names
                .into_iter()
                .map(|name| name.entry_name().clone())
                .collect(),
        )))
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::folder(EntryName::Regex(pattern.into())))
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::folder(EntryName::Wildmatch(pattern.into())))
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneEntry {
        self.within_path_buf(directory.into())
    }

    pub fn alias(&self, name: impl Into<String>) -> EntityAlias {
        EntityAlias::new(self.boxed(), name)
    }
}

impl OneEntryNamed for FileOrFolderNamed {
    fn within_path_buf(&self, directory: PathBuf) -> OneEntry {
        OneEntry::new(self.boxed(), directory)
    }

    fn entry_name(&self) -> &EntryName {
        self.0.entry_name()
    }

    fn entry_type(&self) -> &EntryType {
        self.0.entry_type()
    }

    fn name_alias(&self) -> Option<&str> {
        None
    }

    fn boxed(&self) -> Box<dyn OneEntryNamed> {
        Box::new(self.clone())
    }
}

impl FileOrFolderNamed {
    pub fn exact(name: impl Into<String>) -> Self {
        Self(EntryNamed::any(EntryName::Exact(name.into())))
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self(EntryNamed::any(EntryName::Any(
            names.into_iter().map(|name| name.into()).collect(),
        )))
    }

    pub fn any_named(names: Vec<FileOrFolderNamed>) -> Self {
        Self(EntryNamed::any(EntryName::AnyNamed(
            names
                .into_iter()
                .map(|name| name.entry_name().clone())
                .collect(),
        )))
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::any(EntryName::Regex(pattern.into())))
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::any(EntryName::Wildmatch(pattern.into())))
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneEntry {
        self.within_path_buf(directory.into())
    }

    pub fn alias(&self, name: impl Into<String>) -> EntityAlias {
        EntityAlias::new(Box::new(self.clone()), name)
    }
}

impl ManyEntriesNamed for FilesNamed {
    fn within_path_buf(&self, directory: PathBuf) -> ManyEntries {
        ManyEntries::new(self.boxed(), directory)
    }

    fn entry_name(&self) -> &EntryName {
        self.0.entry_name()
    }

    fn entry_type(&self) -> &EntryType {
        self.0.entry_type()
    }

    fn name_alias(&self) -> Option<&str> {
        None
    }

    fn boxed(&self) -> Box<dyn ManyEntriesNamed> {
        Box::new(self.clone())
    }
}

impl FilesNamed {
    pub fn exact(name: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Exact(name.into())))
    }

    pub fn any(names: Vec<impl Into<String>>) -> Self {
        Self(EntryNamed::file(EntryName::Any(
            names.into_iter().map(|name| name.into()).collect(),
        )))
    }

    #[cfg(feature = "regex")]
    pub fn regex(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Regex(pattern.into())))
    }

    #[cfg(feature = "wildmatch")]
    pub fn wildmatch(pattern: impl Into<String>) -> Self {
        Self(EntryNamed::file(EntryName::Wildmatch(pattern.into())))
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> ManyEntries {
        ManyEntries::new(self.boxed(), directory)
    }
}
