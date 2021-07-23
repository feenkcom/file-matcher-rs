extern crate tempdir;

use file_matcher::{FileNamed, FolderNamed, OneEntryCopier, Result};
use tempdir::TempDir;

#[test]
pub fn copy_one_file_exact() -> Result<()> {
    let tmp_dir = TempDir::new("test")?;

    let file = FileNamed::exact("cat.txt").within("tests/assets");
    let copied = file.copy(tmp_dir.path())?;

    assert_eq!(&copied, &tmp_dir.path().join("cat.txt"));

    std::fs::remove_file(copied)?;

    tmp_dir.close()?;
    Ok(())
}

#[test]
pub fn copy_one_file_exact_alias() -> Result<()> {
    let tmp_dir = TempDir::new("test")?;

    let file = FileNamed::exact("cat.txt")
        .alias("kitty.txt")
        .within("tests/assets");

    let copied = file.copy(tmp_dir.path())?;

    assert_eq!(&copied, &tmp_dir.path().join("kitty.txt"));

    std::fs::remove_file(copied)?;
    tmp_dir.close()?;
    Ok(())
}

#[test]
pub fn copy_one_folder_exact() -> Result<()> {
    let tmp_dir = TempDir::new("test")?;

    let folder = FolderNamed::exact("cat").within("tests/assets");

    let copied = folder.copy(tmp_dir.path())?;

    assert_eq!(&copied, &tmp_dir.path().join("cat"));

    std::fs::remove_dir_all(copied)?;
    tmp_dir.close()?;
    Ok(())
}

#[test]
pub fn copy_one_folder_exact_alias() -> Result<()> {
    let tmp_dir = TempDir::new("test")?;

    let folder = FolderNamed::exact("cat")
        .alias("kitty")
        .within("tests/assets");

    let copied = folder.copy(tmp_dir.path())?;

    assert_eq!(&copied, &tmp_dir.path().join("kitty"));

    std::fs::remove_dir_all(copied)?;
    tmp_dir.close()?;
    Ok(())
}
