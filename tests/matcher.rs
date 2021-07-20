use file_matcher::{BoxError, FileNamed, FilesNamed, OneFileCopier};
use std::ffi::OsStr;
use std::path::PathBuf;

#[test]
pub fn one_exact() -> Result<(), BoxError> {
    let file = FileNamed::exact("cat.txt").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn copy_one_exact() -> Result<(), BoxError> {
    let file = FileNamed::exact("cat.txt").within("tests/assets");

    std::fs::create_dir_all("tests/assets/copy_one_exact/")?;
    let copied = file.copy("tests/assets/copy_one_exact/")?;

    assert_eq!(&copied, &PathBuf::from("tests/assets/copy_one_exact/cat.txt"));

    std::fs::remove_file(copied)?;
    std::fs::remove_dir_all("tests/assets/copy_one_exact/")?;
    Ok(())
}

#[test]
pub fn copy_one_exact_alias() -> Result<(), BoxError> {
    let file = FileNamed::exact("cat.txt").alias("kitty.txt").within("tests/assets");

    std::fs::create_dir_all("tests/assets/copy_one_exact_alias/")?;
    let copied = file.copy("tests/assets/copy_one_exact_alias/")?;

    assert_eq!(&copied, &PathBuf::from("tests/assets/copy_one_exact_alias/kitty.txt"));

    std::fs::remove_file(copied)?;
    std::fs::remove_dir_all("tests/assets/copy_one_exact_alias/")?;
    Ok(())
}

#[test]
pub fn one_any() -> Result<(), BoxError> {
    let file = FileNamed::any(vec!["cat.txt", "bird.txt"])
        .within("tests/assets")
        .find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn one_regex() -> Result<(), BoxError> {
    let file = FileNamed::regex("cat.*").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn one_wildmatch() -> Result<(), BoxError> {
    let file = FileNamed::wildmatch("cat*").within("tests/assets").find()?;
    assert_eq!(file.file_name().unwrap(), "cat.txt");
    Ok(())
}

#[test]
pub fn many_exact() -> Result<(), BoxError> {
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
pub fn many_any() -> Result<(), BoxError> {
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
pub fn many_regex() -> Result<(), BoxError> {
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
pub fn many_wildmatch() -> Result<(), BoxError> {
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
