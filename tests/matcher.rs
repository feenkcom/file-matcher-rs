use file_matcher::{
    FileMatcherError, FileNamed, FileOrFolderNamed, FilesNamed, FolderNamed, Result,
};
use std::ffi::OsStr;

#[test]
pub fn find_one_file_exact() -> Result<()> {
    let file = FileNamed::exact("cat.txt").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn exists_one_file_exact() -> Result<()> {
    let exists = FileNamed::exact("cat.txt")
        .within("tests/assets")
        .exists()?;
    assert!(exists);
    Ok(())
}

#[test]
pub fn does_not_exist_one_file_exact() -> Result<()> {
    let exists = FileNamed::exact("kitty.txt")
        .within("tests/assets")
        .exists()?;
    assert!(!exists);
    Ok(())
}

#[test]
pub fn one_folder_exact() -> Result<()> {
    let folder = FolderNamed::exact("cat").within("tests/assets").find()?;
    assert_eq!(folder.file_name().unwrap(), "cat");
    Ok(())
}

#[test]
pub fn one_file_any() -> Result<()> {
    let file = FileNamed::any(vec!["cat.txt", "bird.txt"])
        .within("tests/assets")
        .find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn one_file_regex() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let file = FileNamed::regex("cat.*").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn one_folder_regex() -> Result<()> {
    let folder = FolderNamed::regex("cat.*").within("tests/assets").find()?;
    assert_eq!(folder.file_name().unwrap(), "cat");
    Ok(())
}

#[test]
pub fn one_file_wildmatch() -> Result<()> {
    let file = FileNamed::wildmatch("cat*").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn one_folder_wildmatch() -> Result<()> {
    let folder = FolderNamed::wildmatch("cat*")
        .within("tests/assets")
        .find()?;
    assert_eq!(folder.file_name().unwrap(), "cat");
    Ok(())
}

#[test]
pub fn one_file_or_file_wildmatch_error() -> Result<()> {
    let entry = FileOrFolderNamed::wildmatch("cat*").within("tests/assets");
    match entry.find() {
        Ok(entry) => {
            panic!("Should fail, but found one {:?}", &entry)
        }
        Err(error) => match &error {
            FileMatcherError::TooMany(failed_entry) => {
                assert_eq!(failed_entry.directory(), entry.directory());
                assert_eq!(failed_entry.entry_name(), entry.entry_name());
                assert_eq!(failed_entry.entry_type(), entry.entry_type());
            }
            _ => {
                panic!("Wrong error type {:?}", error);
            }
        },
    }
    Ok(())
}

#[test]
pub fn many_exact() -> Result<()> {
    let files = FilesNamed::exact("cat.txt").within("tests/assets").find()?;

    let file_names = files
        .iter()
        .map(|each| each.file_name().unwrap())
        .collect::<Vec<&OsStr>>();
    assert_eq!(file_names.len(), 1);
    assert!(file_names.contains(&OsStr::new("cat.txt")));
    Ok(())
}

#[test]
pub fn many_any() -> Result<()> {
    let files = FilesNamed::any(vec!["cat.txt", "dog.txt", "bird.txt"])
        .within("tests/assets")
        .find()?;

    let file_names = files
        .iter()
        .map(|each| each.file_name().unwrap())
        .collect::<Vec<&OsStr>>();
    assert_eq!(file_names.len(), 2);
    assert!(file_names.contains(&OsStr::new("cat.txt")));
    assert!(file_names.contains(&OsStr::new("dog.txt")));
    Ok(())
}

#[test]
pub fn many_regex() -> Result<()> {
    let files = FilesNamed::regex(".*\\.txt")
        .within("tests/assets")
        .find()?;

    let file_names = files
        .iter()
        .map(|each| each.file_name().unwrap())
        .collect::<Vec<&OsStr>>();
    assert_eq!(file_names.len(), 2);
    assert!(file_names.contains(&OsStr::new("cat.txt")));
    assert!(file_names.contains(&OsStr::new("dog.txt")));
    Ok(())
}

#[test]
pub fn many_wildmatch() -> Result<()> {
    let files = FilesNamed::wildmatch("*.txt")
        .within("tests/assets")
        .find()?;

    let file_names = files
        .iter()
        .map(|each| each.file_name().unwrap())
        .collect::<Vec<&OsStr>>();
    assert_eq!(file_names.len(), 2);
    assert!(file_names.contains(&OsStr::new("cat.txt")));
    assert!(file_names.contains(&OsStr::new("dog.txt")));
    Ok(())
}
