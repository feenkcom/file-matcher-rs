use crate::filters::FilesFilter;
use crate::utils::is_readable_file;
use crate::{FilesNamed, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MultipleFilesFilter {
    name: FilesNamed,
    directory: PathBuf,
}

impl MultipleFilesFilter {
    pub fn new(name: FilesNamed, directory: impl Into<PathBuf>) -> Self {
        Self {
            name,
            directory: directory.into(),
        }
    }

    pub fn find(&self) -> Result<Vec<PathBuf>> {
        match &self.name {
            FilesNamed::Exact(name) => {
                let file = self.directory.join(name);
                if is_readable_file(&file) {
                    Ok(vec![file])
                } else {
                    Ok(vec![])
                }
            }
            FilesNamed::Any(names) => {
                let files = names
                    .iter()
                    .map(|each| self.directory.join(each))
                    .filter(|each| is_readable_file(each))
                    .collect::<Vec<PathBuf>>();

                Ok(files)
            }
            #[cfg(feature = "regex")]
            FilesNamed::Regex(regex_pattern) => {
                let files = crate::finders::regex_finder::find_files_in_directory_matching(
                    regex_pattern,
                    &self.directory,
                )?;
                Ok(files)
            }
            #[cfg(feature = "wildmatch")]
            FilesNamed::Wildmatch(wildmatch_pattern) => {
                let files = crate::finders::wildmatch_finder::find_files_in_directory_matching(
                    wildmatch_pattern,
                    &self.directory,
                )?;
                Ok(files)
            }
        }
    }
}

impl FilesFilter for MultipleFilesFilter {
    fn all(&self) -> Result<Vec<PathBuf>> {
        self.find()
    }

    fn into_filter(self) -> Box<dyn FilesFilter> {
        Box::new(self)
    }
}
