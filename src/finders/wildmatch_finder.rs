#[cfg(not(feature = "wildmatch"))]
compile_error!("Please select a wildmatch feature to build with wildmatch support");

use crate::utils::readable_entries_in_folder;
use crate::{EntryType, Result};
use std::path::{Path, PathBuf};
use wildmatch::WildMatch;

pub(crate) fn find_entries_in_directory_matching(
    entity_type: &EntryType,
    entity_name_wildmatch: &str,
    directory: impl AsRef<Path>,
) -> Result<Vec<PathBuf>> {
    let compiled_wildmatch = WildMatch::new(entity_name_wildmatch);

    let files = readable_entries_in_folder(entity_type, directory)
        .into_iter()
        .filter(|each_path| {
            each_path.file_name().map_or(false, |file_name| {
                file_name
                    .to_str()
                    .map_or(false, |file_name| compiled_wildmatch.matches(file_name))
            })
        })
        .collect::<Vec<PathBuf>>();
    Ok(files)
}
