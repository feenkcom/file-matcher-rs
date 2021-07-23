use crate::EntryType;
use std::fs::symlink_metadata;
use std::path::{Path, PathBuf};

pub(crate) fn readable_entries_in_folder(
    entity_type: &EntryType,
    folder: impl AsRef<Path>,
) -> Vec<PathBuf> {
    folder.as_ref().read_dir().map_or(vec![], |dir| {
        dir.filter(|each_entry| each_entry.is_ok())
            .map(|each_entry| each_entry.unwrap())
            .map(|each_entry| each_entry.path())
            .filter(|each_path| is_readable_entry(entity_type, each_path))
            .collect::<Vec<PathBuf>>()
    })
}

/// Returns true if a given Path entry is an entry of the given type.
/// returns false if a given entry is not readable or does not exist
pub(crate) fn is_readable_entry(entity_type: &EntryType, entry: impl AsRef<Path>) -> bool {
    let entry = entry.as_ref();
    match entity_type {
        EntryType::File => is_readable_file(entry),
        EntryType::Folder => is_readable_folder(entry),
        EntryType::Any => is_readable_file(entry) || is_readable_folder(entry),
    }
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

/// Returns true if a given Path entry is a folder (not a symlink and not a file),
/// returns false if a given entry is not readable or does not exist
pub(crate) fn is_readable_folder(entry: impl AsRef<Path>) -> bool {
    if let Ok(metadata) = symlink_metadata(entry.as_ref()) {
        let file_type = metadata.file_type();
        file_type.is_dir()
    } else {
        false
    }
}
