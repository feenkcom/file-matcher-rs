#[cfg(not(feature = "regex"))]
compile_error!("Please select a regex feature to build with regex support");

use crate::utils::readable_files_in_folder;
use crate::Result;
use regex::Regex;
use std::path::{Path, PathBuf};

pub(crate) fn find_files_in_directory_matching(
    file_name_regex: &str,
    directory: impl AsRef<Path>,
) -> Result<Vec<PathBuf>> {
    let compiled_regex = Regex::new(file_name_regex)?;

    let files = readable_files_in_folder(directory)
        .into_iter()
        .filter(|each_path| {
            each_path.file_name().map_or(false, |file_name| {
                file_name
                    .to_str()
                    .map_or(false, |file_name| compiled_regex.is_match(file_name))
            })
        })
        .collect::<Vec<PathBuf>>();
    Ok(files)
}
