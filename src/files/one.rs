use crate::files::Files;
use crate::utils::is_readable_file;
use crate::{Error, FileNamed, Result, OneFileNamed};
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct OneFile {
    name: Box<dyn OneFileNamed>,
    directory: PathBuf,
}

impl OneFile {
    pub fn new(name: Box<dyn OneFileNamed>, directory: impl Into<PathBuf>) -> Self {
        Self {
            name,
            directory: directory.into(),
        }
    }

    pub fn name(&self) -> &dyn OneFileNamed {
        self.name.as_ref()
    }

    pub fn find(&self) -> Result<PathBuf> {
        match self.name.name_type() {
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
                    .filter(|each| is_readable_file(each.as_path()))
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

impl Clone for OneFile {
    fn clone(&self) -> Self {
        Self::new(self.name.boxed(), self.directory.clone())
    }
}

impl Files for OneFile {
    fn all(&self) -> Result<Vec<PathBuf>> {
        self.find().map(|file| vec![file])
    }

    fn into_files(self) -> Box<dyn Files> {
        Box::new(self)
    }
}

impl From<OneFile> for PathBuf {
    fn from(filter: OneFile) -> Self {
        PathBuf::from(&filter)
    }
}

impl From<&OneFile> for PathBuf {
    fn from(filter: &OneFile) -> Self {
        filter
            .find()
            .expect("Could not find exactly one matching file")
    }
}

impl From<&OneFile> for Result<OsString> {
    fn from(filter: &OneFile) -> Self {
        filter.as_os_string()
    }
}

impl From<OneFile> for Result<OsString> {
    fn from(filter: OneFile) -> Self {
        (&filter).into()
    }
}

impl From<&OneFile> for Result<String> {
    fn from(filter: &OneFile) -> Self {
        filter.as_string()
    }
}

impl From<OneFile> for Result<String> {
    fn from(filter: OneFile) -> Self {
        (&filter).into()
    }
}

impl From<&OneFile> for Result<Vec<u8>> {
    fn from(filter: &OneFile) -> Self {
        filter.as_bytes()
    }
}

impl From<OneFile> for Result<Vec<u8>> {
    fn from(filter: OneFile) -> Self {
        (&filter).into()
    }
}
