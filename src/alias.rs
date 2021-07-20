use crate::{FileNamed, OneFileNamed, OneFile};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileAlias {
    file_named: FileNamed,
    alias: String,
}

impl FileAlias {
    pub fn new(file_named: FileNamed, alias: impl Into<String>) -> Self {
        Self {
            file_named,
            alias: alias.into(),
        }
    }

    pub fn within(&self, directory: impl Into<PathBuf>) -> OneFile {
        self.within_path_buf(directory.into())
    }
}

impl OneFileNamed for FileAlias {
    fn within_path_buf(&self, directory: PathBuf) -> OneFile {
        OneFile::new(self.boxed(), directory)
    }

    fn name_type(&self) -> &FileNamed {
        self.file_named.name_type()
    }

    fn name_alias(&self) -> Option<&str> {
        Some(self.alias.as_str())
    }

    fn boxed(&self) -> Box<dyn OneFileNamed> {
        Box::new(self.clone())
    }
}