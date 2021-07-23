use crate::utils::is_readable_entry;
use crate::{EntryName, EntryType, FileMatcherError, Result};
use std::ffi::OsString;
use std::fmt::Debug;
use std::io::Read;
use std::path::{Path, PathBuf};

pub trait OneEntryNamed: Debug {
    fn within_path_buf(&self, directory: PathBuf) -> OneEntry;
    fn entry_name(&self) -> &EntryName;
    fn entry_type(&self) -> &EntryType;
    fn name_alias(&self) -> Option<&str>;
    fn boxed(&self) -> Box<dyn OneEntryNamed>;
}

#[derive(Debug)]
pub struct OneEntry {
    entry_named: Box<dyn OneEntryNamed>,
    directory: PathBuf,
}

impl OneEntry {
    pub fn new(entry_named: Box<dyn OneEntryNamed>, directory: impl Into<PathBuf>) -> Self {
        Self {
            entry_named,
            directory: directory.into(),
        }
    }

    pub fn entry(&self) -> &dyn OneEntryNamed {
        self.entry_named.as_ref()
    }

    pub fn entry_type(&self) -> &EntryType {
        self.entry().entry_type()
    }

    pub fn entry_name(&self) -> &EntryName {
        self.entry().entry_name()
    }

    pub fn directory(&self) -> &Path {
        self.directory.as_path()
    }

    /// Return true if there exists exactly one entry of the specified type and name,
    /// false otherwise
    pub fn exists(&self) -> Result<bool> {
        match self.find() {
            Ok(path) => Ok(path.exists()),
            Err(error) => match &error {
                FileMatcherError::NotExists(_) => Ok(false),
                _ => error.into(),
            },
        }
    }

    /// Try to find an exactly one entry of the specified type and name
    pub fn find(&self) -> Result<PathBuf> {
        let entity_type = self.entry_named.entry_type();

        match self.entry_named.entry_name() {
            EntryName::Exact(name) => {
                let entry = self.directory.join(name);
                if is_readable_entry(entity_type, &entry) {
                    Ok(entry)
                } else {
                    FileMatcherError::NotExists(self.clone()).into()
                }
            }
            EntryName::Any(names) => {
                let entries = names
                    .iter()
                    .map(|each| self.directory.join(each))
                    .filter(|each| is_readable_entry(entity_type, each.as_path()))
                    .collect::<Vec<PathBuf>>();

                match entries.len() {
                    0 => FileMatcherError::NotExists(self.clone()).into(),
                    1 => Ok(entries.first().unwrap().to_owned()),
                    _ => FileMatcherError::TooMany(self.clone()).into(),
                }
            }
            #[cfg(feature = "regex")]
            EntryName::Regex(regex_pattern) => {
                let entries = crate::finders::regex_finder::find_entries_in_directory_matching(
                    entity_type,
                    regex_pattern,
                    &self.directory,
                )?;
                match entries.len() {
                    0 => FileMatcherError::NotExists(self.clone()).into(),
                    1 => Ok(entries.first().unwrap().to_owned()),
                    _ => FileMatcherError::TooMany(self.clone()).into(),
                }
            }
            #[cfg(feature = "wildmatch")]
            EntryName::Wildmatch(wildmatch_pattern) => {
                let entries = crate::finders::wildmatch_finder::find_entries_in_directory_matching(
                    entity_type,
                    wildmatch_pattern,
                    &self.directory,
                )?;
                match entries.len() {
                    0 => FileMatcherError::NotExists(self.clone()).into(),
                    1 => Ok(entries.first().unwrap().to_owned()),
                    _ => FileMatcherError::TooMany(self.clone()).into(),
                }
            }
        }
    }

    pub fn as_path_buf(&self) -> Result<PathBuf> {
        self.find()
    }

    pub fn as_os_string(&self) -> Result<OsString> {
        let path = self.find()?;
        match path.file_name() {
            None => FileMatcherError::NotReadable(path.clone()).into(),
            Some(file_name) => Ok(file_name.to_os_string()),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        let file_name = self.as_os_string()?;
        match file_name.to_str() {
            None => FileMatcherError::InvalidUnicode(file_name).into(),
            Some(file_name) => Ok(file_name.to_owned()),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let path = self.find()?;
        let mut file = std::fs::File::open(path.as_path())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl Clone for OneEntry {
    fn clone(&self) -> Self {
        Self::new(self.entry_named.boxed(), self.directory.clone())
    }
}

impl From<OneEntry> for PathBuf {
    fn from(filter: OneEntry) -> Self {
        PathBuf::from(&filter)
    }
}

impl From<&OneEntry> for PathBuf {
    fn from(filter: &OneEntry) -> Self {
        filter
            .find()
            .expect("Could not find exactly one matching file")
    }
}

impl From<&OneEntry> for Result<OsString> {
    fn from(filter: &OneEntry) -> Self {
        filter.as_os_string()
    }
}

impl From<OneEntry> for Result<OsString> {
    fn from(filter: OneEntry) -> Self {
        (&filter).into()
    }
}

impl From<&OneEntry> for Result<String> {
    fn from(filter: &OneEntry) -> Self {
        filter.as_string()
    }
}

impl From<OneEntry> for Result<String> {
    fn from(filter: OneEntry) -> Self {
        (&filter).into()
    }
}

impl From<&OneEntry> for Result<Vec<u8>> {
    fn from(filter: &OneEntry) -> Self {
        filter.as_bytes()
    }
}

impl From<OneEntry> for Result<Vec<u8>> {
    fn from(filter: OneEntry) -> Self {
        (&filter).into()
    }
}
