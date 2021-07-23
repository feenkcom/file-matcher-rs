use crate::OneEntry;
use std::error;
use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, FileMatcherError>;

#[derive(Debug)]
pub enum FileMatcherError {
    TooMany(OneEntry),
    NotExists(OneEntry),
    NotReadable(PathBuf),
    InvalidUnicode(OsString),
    IoError(std::io::Error),
    #[cfg(feature = "fs_extra")]
    FsExtraError(fs_extra::error::Error),
    #[cfg(feature = "regex")]
    RegexError(regex::Error),
}

impl fmt::Display for FileMatcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileMatcherError::TooMany(entry) => {
                write!(
                    f,
                    "Found more than one {:?} named {:?} in {:?}",
                    entry.entry_type(),
                    entry.entry_name(),
                    entry.directory()
                )
            }
            FileMatcherError::NotExists(entry) => {
                write!(
                    f,
                    "Could not find {:?} named {:?} in {:?}",
                    entry.entry_type(),
                    entry.entry_name(),
                    entry.directory()
                )
            }
            FileMatcherError::IoError(error) => {
                write!(f, "Failed to perform IO operation {:?}", error)
            }
            #[cfg(feature = "fs_extra")]
            FileMatcherError::FsExtraError(error) => {
                write!(f, "Failed to perform IO operation {:?}", error)
            }
            #[cfg(feature = "regex")]
            FileMatcherError::RegexError(error) => {
                write!(f, "Failed to create regex {:?}", error)
            }
            FileMatcherError::NotReadable(path) => {
                write!(f, "Failed to read {:?}", path)
            }
            FileMatcherError::InvalidUnicode(file_name) => {
                write!(f, "Failed to convert {:?} to Unicode", file_name)
            }
        }
    }
}

impl error::Error for FileMatcherError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            FileMatcherError::IoError(ref e) => Some(e),
            #[cfg(feature = "regex")]
            FileMatcherError::RegexError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<regex::Error> for FileMatcherError {
    fn from(err: regex::Error) -> FileMatcherError {
        FileMatcherError::RegexError(err)
    }
}

impl From<std::io::Error> for FileMatcherError {
    fn from(err: std::io::Error) -> FileMatcherError {
        FileMatcherError::IoError(err)
    }
}

impl<T> From<FileMatcherError> for std::result::Result<T, FileMatcherError> {
    fn from(error: FileMatcherError) -> Self {
        Err(error)
    }
}

unsafe impl Sync for FileMatcherError {}
unsafe impl Send for FileMatcherError {}
