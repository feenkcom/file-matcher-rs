use std::fs::symlink_metadata;
use std::path::{Path, PathBuf};

pub(crate) fn readable_files_in_folder(folder: impl AsRef<Path>) -> Vec<PathBuf> {
    folder.as_ref().read_dir().map_or(vec![], |dir| {
        dir.filter(|each_entry| each_entry.is_ok())
            .map(|each_entry| each_entry.unwrap())
            .map(|each_entry| each_entry.path())
            .filter(|each_path| is_readable_file(each_path))
            .collect::<Vec<PathBuf>>()
    })
}

/// Returns true if a given Path entry is a file (not a symlink and not a folder),
/// returns false if a given entry is not readable or does not exist
pub(crate) fn is_readable_file(entry: impl AsRef<Path>) -> bool {
    if let Ok(metadata) = symlink_metadata(entry.as_ref()) {
        let file_type = metadata.file_type();
        file_type.is_file()
    } else {
        false
    }
}
