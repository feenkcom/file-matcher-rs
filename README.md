# file-matcher
A Rust library to search files and folders based on the name pattern (regex, wildcard, exact).

### Search

Use `FileNamed` to search for exactly one file matching the name pattern. Returns an `Error` if none or more than one file was found.
```
FileNamed::regex("cat.*")
    .within("tests/assets")
    .find()?
```

Use `FolderNamed` to search for exactly one folder matching the name pattern. Returns an `Error` if none or more than one folder was found.
```
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .find()?
```

### Existence

Check if a file exists:
```
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .exists()?
```

Check if a folder exists:
```
FolderNamed::wildmatch("cat*")
    .within("tests/assets")
    .exists()?
```

### Copy

Find and copy a file matching a name pattern to `destination` folder under the same name:
```
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .copy("destination")?
```

Find and copy a file matching a name pattern to `destination` folder as `kitty.txt`:
```
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .copy(Path::new("destination").join("kitty.txt"))?
```

Alternatively, assign an alias for copy/move operations.
The following will find a file matching a given pattern name and will copy it into the `destination` folder under the `kitty.txt` name:
```
FileNamed::wildmatch("cat*")
    .alias("kitty.txt")
    .within("tests/assets")
    .copy("destination")?
```