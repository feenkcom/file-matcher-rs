#[cfg(not(feature = "mover"))]
compile_error!("Please select a mover feature to build with mover support");

use crate::Result;
use crate::{Error, OneFile};
use std::path::Path;

pub trait OneFileMover {
    fn r#move(&self, destination: impl AsRef<Path>) -> Result<()>;
}

impl OneFileMover for OneFile {
    fn r#move(&self, destination: impl AsRef<Path>) -> Result<()> {
        let destination = destination.as_ref();
        let file = self.as_path_buf()?;

        return if let Some(alias) = self.name().name_alias() {
            Ok(move_file(&file, destination, alias)?)
        } else if let Some(file_name) = file.file_name() {
            if let Some(file_name) = file_name.to_str() {
                Ok(move_file(&file, destination, file_name)?)
            } else {
                Err(Error::new("Could not convert a file name to Unicode").boxed())
            }
        } else {
            Err(Error::new("Could not get a file name").boxed())
        };
    }
}

fn move_file(from: impl AsRef<Path>, to: impl AsRef<Path>, file_name: &str) -> Result<()> {
    let destination = to.as_ref();

    let destination = if destination.is_dir() {
        destination.join(file_name)
    } else {
        destination.to_path_buf()
    };

    std::fs::copy(&from, &destination)?;
    std::fs::remove_file(&from)?;
    Ok(())
}
