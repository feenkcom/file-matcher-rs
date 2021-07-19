use crate::filters::FilesFilter;
use crate::utils::is_readable_file;
use crate::{Error, FileNamed, Result};
use std::ffi::OsString;
use std::io::Read;
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

    pub fn find(&self) -> Result<PathBuf> {
        match &self.name {
            FileNamed::Exact(name) => {
                let file = self.directory.join(name);
                if is_readable_file(&file) {
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
                    .filter(|each| is_readable_file(each))
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

    pub fn as_path_buf(&self) -> Result<PathBuf> {
        self.find()
    }

    pub fn as_os_string(&self) -> Result<OsString> {
        let path = self.find()?;
        match path.file_name() {
            None => Err(Error::new(format!("Could not get the file name of {:?}", &path)).boxed()),
            Some(file_name) => Ok(file_name.to_os_string()),
        }
    }

    pub fn as_string(&self) -> Result<String> {
        let file_name = self.as_os_string()?;
        match file_name.to_str() {
            None => Err(Error::new(format!(
                "Could not convert the file name to Unicode String {:?}",
                &file_name
            ))
            .boxed()),
            Some(file_name) => Ok(file_name.to_owned()),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let path = self.find()?;
        let mut file = std::fs::File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl FilesFilter for OneFileFilter {
    fn all(&self) -> Result<Vec<PathBuf>> {
        self.find().map(|file| vec![file])
    }

    fn into_filter(self) -> Box<dyn FilesFilter> {
        Box::new(self)
    }
}

impl From<OneFileFilter> for PathBuf {
    fn from(filter: OneFileFilter) -> Self {
        PathBuf::from(&filter)
    }
}

impl From<&OneFileFilter> for PathBuf {
    fn from(filter: &OneFileFilter) -> Self {
        filter
            .find()
            .expect("Could not find exactly one matching file")
    }
}

impl From<&OneFileFilter> for Result<OsString> {
    fn from(filter: &OneFileFilter) -> Self {
        filter.as_os_string()
    }
}

impl From<OneFileFilter> for Result<OsString> {
    fn from(filter: OneFileFilter) -> Self {
        (&filter).into()
    }
}

impl From<&OneFileFilter> for Result<String> {
    fn from(filter: &OneFileFilter) -> Self {
        filter.as_string()
    }
}

impl From<OneFileFilter> for Result<String> {
    fn from(filter: OneFileFilter) -> Self {
        (&filter).into()
    }
}

impl From<&OneFileFilter> for Result<Vec<u8>> {
    fn from(filter: &OneFileFilter) -> Self {
        filter.as_bytes()
    }
}

impl From<OneFileFilter> for Result<Vec<u8>> {
    fn from(filter: OneFileFilter) -> Self {
        (&filter).into()
    }
}
