#[cfg(not(feature = "wildmatch"))]
compile_error!("Please select a wildmatch feature to build with wildmatch support");

use crate::utils::readable_files_in_folder;
use crate::Result;
use std::path::{Path, PathBuf};
use wildmatch::WildMatch;

pub(crate) fn find_files_in_directory_matching(
    file_name_wildmatch: &str,
    directory: impl AsRef<Path>,
) -> Result<Vec<PathBuf>> {
    let compiled_wildmatch = WildMatch::new(file_name_wildmatch);

    let files = readable_files_in_folder(directory)
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
