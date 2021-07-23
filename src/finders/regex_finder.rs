#[cfg(not(feature = "regex"))]
compile_error!("Please select a regex feature to build with regex support");

use crate::utils::readable_entries_in_folder;
use crate::{EntryType, Result};
use regex::Regex;
use std::path::{Path, PathBuf};

pub(crate) fn find_entries_in_directory_matching(
    entity_type: &EntryType,
    entity_name_regex: &str,
    directory: impl AsRef<Path>,
) -> Result<Vec<PathBuf>> {
    let compiled_regex = Regex::new(entity_name_regex)?;

    let files = readable_entries_in_folder(entity_type, directory)
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
