#[cfg(not(feature = "copier"))]
compile_error!("Please select a copier feature to build with copier support");

use crate::{FileMatcherError, OneEntry, Result};
use fs_extra::dir::CopyOptions;
use std::path::{Path, PathBuf};

pub trait OneEntryCopier {
    fn copy(&self, destination: impl AsRef<Path>) -> Result<PathBuf>;
}

impl OneEntryCopier for OneEntry {
    fn copy(&self, destination: impl AsRef<Path>) -> Result<PathBuf> {
        let destination = destination.as_ref();
        let entry = self.as_path_buf()?;

        return if let Some(alias) = self.entry().name_alias() {
            Ok(copy_entry(&entry, destination, alias)?)
        } else if let Some(file_name) = entry.file_name() {
            if let Some(file_name) = file_name.to_str() {
                Ok(copy_entry(&entry, destination, file_name)?)
            } else {
                FileMatcherError::InvalidUnicode(file_name.to_os_string()).into()
            }
        } else {
            FileMatcherError::NotReadable(entry.clone()).into()
        };
    }
}

fn copy_entry(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let from = from.as_ref();
    if from.is_file() {
        copy_file(from, to, file_name)
    } else if from.is_dir() {
        copy_folder(from, to, file_name)
    } else {
        FileMatcherError::NotReadable(from.to_path_buf()).into()
    }
}

fn copy_file(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let destination = to.as_ref();

    let destination = if destination.is_dir() {
        destination.join(file_name)
    } else {
        destination.to_path_buf()
    };

    std::fs::copy(&from, &destination)?;
    Ok(destination)
}

fn copy_folder(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let destination = to.as_ref();

    let destination = destination.join(file_name);

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    fs_extra::dir::copy(&from, &destination, &options).map_err(FileMatcherError::FsExtraError)?;

    Ok(destination)
}
