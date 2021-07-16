# file-matcher-rs
A Rust library to search files based on the name pattern (regex, wildcard, exact).

### Examples

Use `FileNamed` to search for exactly one file matching the name pattern. Returns an `Error` if none or more than one file was found.
```
FileNamed::Regex("cat.*")
    .within("tests/assets")
    .find()?
```

Use `FilesNamed` to find any amount of files matching the name pattern.
```
FilesNamed::Wildmatch("*.txt")
    .within("tests/assets")
    .find()?
```
