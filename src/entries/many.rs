use crate::utils::is_readable_entry;
use crate::{EntryName, EntryType, Result};
use std::fmt::Debug;
use std::path::PathBuf;

pub trait ManyEntriesNamed: Debug {
    fn within_path_buf(&self, directory: PathBuf) -> ManyEntries;
    fn entry_name(&self) -> &EntryName;
    fn entry_type(&self) -> &EntryType;
    fn name_alias(&self) -> Option<&str>;
    fn boxed(&self) -> Box<dyn ManyEntriesNamed>;
}

#[derive(Debug)]
pub struct ManyEntries {
    entries_named: Box<dyn ManyEntriesNamed>,
    directory: PathBuf,
}

impl ManyEntries {
    pub fn new(entries_named: Box<dyn ManyEntriesNamed>, directory: impl Into<PathBuf>) -> Self {
        Self {
            entries_named,
            directory: directory.into(),
        }
    }

    pub fn find(&self) -> Result<Vec<PathBuf>> {
        let entry_type = self.entries_named.entry_type();

        match self.entries_named.entry_name() {
            EntryName::Exact(name) => {
                let entry = self.directory.join(name);
                if is_readable_entry(entry_type, &entry) {
                    Ok(vec![entry])
                } else {
                    Ok(vec![])
                }
            }
            EntryName::Any(names) => {
                let entries = names
                    .iter()
                    .map(|each| self.directory.join(each))
                    .filter(|each| is_readable_entry(entry_type, each.as_path()))
                    .collect::<Vec<PathBuf>>();

                Ok(entries)
            }
            #[cfg(feature = "regex")]
            EntryName::Regex(regex_pattern) => {
                let entries = crate::finders::regex_finder::find_entries_in_directory_matching(
                    entry_type,
                    regex_pattern,
                    &self.directory,
                )?;
                Ok(entries)
            }
            #[cfg(feature = "wildmatch")]
            EntryName::Wildmatch(wildmatch_pattern) => {
                let entries = crate::finders::wildmatch_finder::find_entries_in_directory_matching(
                    entry_type,
                    wildmatch_pattern,
                    &self.directory,
                )?;
                Ok(entries)
            }
        }
    }
}

impl Clone for ManyEntries {
    fn clone(&self) -> Self {
        Self::new(self.entries_named.boxed(), self.directory.clone())
    }
}
