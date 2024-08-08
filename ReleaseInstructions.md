* Install _cargo release_ version v0.25.10 or higher (see [here](https://github.com/sunng87/cargo-release))
* Update CHANGELOG.md with changes for this release. Make sure it has '[Unreleased] - ReleaseDate' section in it. For example:
```
## [Unreleased] - ReleaseDate
### Changed
- Fixed major versions of dependencies
```
* Review the output of following command to make sure it outputs commands you expected:
```
cargo release patch
```
* If previous command is successful, run the following command to publish the crate:
```
cargo release --execute patch
```

### Note

Replace '_patch_' in commands above with '_minor_' or '_major_' if you are releasing a minor or major version
