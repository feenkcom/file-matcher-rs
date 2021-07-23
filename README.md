# file-matcher
[![docs](https://docs.rs/file-matcher/badge.svg)](https://docs.rs/file-matcher)
[![crate](https://img.shields.io/crates/v/file-matcher.svg?color=orange)](https://crates.io/crates/file-matcher)
[![license](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust library to search files and folders based on the name pattern (regex, wildcard, exact).

### Features
* `regex` - adds regex support using [Regex crate](https://crates.io/crates/regex)
* `wildmatch` - adds a wildcard matching using [Wildmatch crate](https://crates.io/crates/wildmatch)
* `copier` - allows users to copy declared files and folders, uses [fs_extra crate](https://crates.io/crates/fs_extra)
* `mover` - allows users to move declared files and folders, uses [fs_extra crate](https://crates.io/crates/fs_extra)

### Search

Use `FileNamed` to search for exactly one file matching the name pattern. Returns an `Error` if none or more than one file was found.
```rust
FileNamed::regex("cat.*")
    .within("tests/assets")
    .find()?
```

Use `FolderNamed` to search for exactly one folder matching the name pattern. Returns an `Error` if none or more than one folder was found.
```rust
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .find()?
```

### Existence

Check if a file exists:
```rust
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .exists()?
```

Check if a folder exists:
```rust
FolderNamed::wildmatch("cat*")
    .within("tests/assets")
    .exists()?
```

### Copy

Find and copy a file matching a name pattern to `destination` folder under the same name:
```rust
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .copy("destination")?
```

Find and copy a file matching a name pattern to `destination` folder as `kitty.txt`:
```rust
FileNamed::wildmatch("cat*")
    .within("tests/assets")
    .copy(Path::new("destination").join("kitty.txt"))?
```

Alternatively, assign an alias for copy/move operations.
The following will find a file matching a given pattern name and will copy it into the `destination` folder under the `kitty.txt` name:
```rust
FileNamed::wildmatch("cat*")
    .alias("kitty.txt")
    .within("tests/assets")
    .copy("destination")?
```