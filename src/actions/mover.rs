#[cfg(not(feature = "mover"))]
compile_error!("Please select a mover feature to build with mover support");

use crate::Result;
use crate::{FileMatcherError, OneEntry};
use fs_extra::dir::CopyOptions;
use std::path::{Path, PathBuf};

pub trait OneEntryMover {
    fn r#move(&self, destination: impl AsRef<Path>) -> Result<PathBuf>;
}

impl OneEntryMover for OneEntry {
    fn r#move(&self, destination: impl AsRef<Path>) -> Result<PathBuf> {
        let destination = destination.as_ref();
        let file = self.as_path_buf()?;

        return if let Some(alias) = self.entry().name_alias() {
            Ok(move_entry(&file, destination, alias)?)
        } else if let Some(file_name) = file.file_name() {
            if let Some(file_name) = file_name.to_str() {
                Ok(move_entry(&file, destination, file_name)?)
            } else {
                FileMatcherError::InvalidUnicode(file_name.to_os_string()).into()
            }
        } else {
            FileMatcherError::NotReadable(file.clone()).into()
        };
    }
}

fn move_entry(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let from = from.as_ref();
    if from.is_file() {
        move_file(from, to, file_name)
    } else if from.is_dir() {
        move_folder(from, to, file_name)
    } else {
        FileMatcherError::NotReadable(from.to_path_buf()).into()
    }
}

fn move_file(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let destination = to.as_ref();

    let destination = if destination.is_dir() {
        destination.join(file_name)
    } else {
        destination.to_path_buf()
    };

    std::fs::copy(&from, &destination)?;
    std::fs::remove_file(&from)?;
    Ok(destination)
}

fn move_folder(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<PathBuf> {
    let destination = to.as_ref();

    let destination = destination.join(file_name);

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    fs_extra::dir::copy(&from, &destination, &options).map_err(FileMatcherError::FsExtraError)?;

    std::fs::remove_dir_all(&from)?;

    Ok(destination)
}
