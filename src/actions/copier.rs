#[cfg(not(feature = "copier"))]
compile_error!("Please select a copier feature to build with copier support");

use crate::Result;
use crate::{Error, OneFile};
use std::path::{Path, PathBuf};

pub trait OneFileCopier {
    fn copy(&self, destination: impl AsRef<Path>) -> Result<PathBuf>;
}

impl OneFileCopier for OneFile {
    fn copy(&self, destination: impl AsRef<Path>) -> Result<PathBuf> {
        let destination = destination.as_ref();
        let file = self.as_path_buf()?;

        return if let Some(alias) = self.name().name_alias() {
            Ok(copy_file(&file, destination, alias)?)
        } else if let Some(file_name) = file.file_name() {
            if let Some(file_name) = file_name.to_str() {
                Ok(copy_file(&file, destination, file_name)?)
            } else {
                Err(Error::new("Could not convert a file name to Unicode").boxed())
            }
        } else {
            Err(Error::new("Could not get a file name").boxed())
        };
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
