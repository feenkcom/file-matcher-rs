use crate::{EntryName, EntryType, OneEntry, OneEntryNamed};
use std::path::PathBuf;

#[derive(Debug)]
pub struct EntityAlias {
    entity_named: Box<dyn OneEntryNamed>,
    alias: String,
}

impl Clone for EntityAlias {
    fn clone(&self) -> Self {
        Self {
            entity_named: self.entity_named.boxed(),
            alias: self.alias.clone(),
        }
    }
}

impl EntityAlias {
    pub fn new(entity_named: Box<dyn OneEntryNamed>, alias: impl Into<String>) -> Self {
        Self {
            entity_named,
            alias: alias.into(),
        }
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneEntry {
        self.within_path_buf(directory.into())
    }
}

impl OneEntryNamed for EntityAlias {
    fn within_path_buf(&self, directory: PathBuf) -> OneEntry {
        OneEntry::new(self.boxed(), directory)
    }

    fn entry_name(&self) -> &EntryName {
        self.entity_named.entry_name()
    }

    fn entry_type(&self) -> &EntryType {
        self.entity_named.entry_type()
    }

    fn name_alias(&self) -> Option<&str> {
        Some(self.alias.as_str())
    }

    fn boxed(&self) -> Box<dyn OneEntryNamed> {
        Box::new(self.clone())
    }
}
