* Install cargo release (see [here](https://github.com/sunng87/cargo-release))
* Update CHANGELOG.md with changes for this release. Make sure it has '[Unreleased] - ReleaseDate' section in it. For example:
```
## [Unreleased] - ReleaseDate
### Changed
- Fixed major versions of dependencies
```
* Review the output of following command to make sure it outputs commands you expected:
```
cargo release patch --dry-run
```
* Run
```
cargo release patch
```

### Note

Replace '_patch_' in commands above with '_minor_' or '_major_' if you are releasing a minor or major version
