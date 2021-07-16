use crate::{BoxError, Error, FileNamed};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OneFileFilter {
    name: FileNamed,
    directory: PathBuf,
}

impl OneFileFilter {
    pub fn new(name: FileNamed, directory: impl Into<PathBuf>) -> Self {
        Self {
            name,
            directory: directory.into(),
        }
    }

    pub fn find(&self) -> Result<PathBuf, BoxError> {
        match &self.name {
            FileNamed::Exact(name) => {
                let file = self.directory.join(name);
                if file.exists() {
                    Ok(file)
                } else {
                    Err(Error::new(format!(
                        "File named {} does not exist in {:?}",
                        name, &self.directory
                    ))
                    .boxed())
                }
            }
            FileNamed::Any(names) => {
                let files = names
                    .iter()
                    .map(|each| self.directory.join(each))
                    .filter(|each| each.exists())
                    .collect::<Vec<PathBuf>>();

                match files.len() {
                    0 => Err(Error::new(format!(
                        "Could not find any file out of {:?} in {:?}",
                        names, &self.directory
                    ))
                    .boxed()),
                    1 => Ok(files.first().unwrap().to_owned()),
                    len => Err(Error::new(format!(
                        "Found more than 1 ({}) files out of {:?} in {:?}",
                        len, names, &self.directory
                    ))
                    .boxed()),
                }
            }
            #[cfg(feature = "regex")]
            FileNamed::Regex(regex_pattern) => {
                let files = crate::finders::regex_finder::find_files_in_directory_matching(
                    regex_pattern,
                    &self.directory,
                )?;
                match files.len() {
                    0 => Err(Error::new(format!(
                        "Could not find any file matching {:?} in {:?}",
                        regex_pattern, &self.directory
                    ))
                    .boxed()),
                    1 => Ok(files.first().unwrap().to_owned()),
                    len => Err(Error::new(format!(
                        "Found more than 1 ({}) files matching {:?} in {:?}",
                        len, regex_pattern, &self.directory
                    ))
                    .boxed()),
                }
            }
            #[cfg(feature = "wildmatch")]
            FileNamed::Wildmatch(wildmatch_pattern) => {
                let files = crate::finders::wildmatch_finder::find_files_in_directory_matching(
                    wildmatch_pattern,
                    &self.directory,
                )?;
                match files.len() {
                    0 => Err(Error::new(format!(
                        "Could not find any file matching {:?} in {:?}",
                        wildmatch_pattern, &self.directory
                    ))
                    .boxed()),
                    1 => Ok(files.first().unwrap().to_owned()),
                    len => Err(Error::new(format!(
                        "Found more than 1 ({}) files matching {:?} in {:?}",
                        len, wildmatch_pattern, &self.directory
                    ))
                    .boxed()),
                }
            }
        }
    }
}

impl From<OneFileFilter> for PathBuf {
    fn from(filter: OneFileFilter) -> Self {
        filter
            .find()
            .expect("Could not find exactly one matching file")
    }
}
