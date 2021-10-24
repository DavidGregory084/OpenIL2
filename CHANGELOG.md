# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fixed the [CHANGELOG](./CHANGELOG.md) links after the GitHub migration.

## [0.1.3]

### Added

- Added a project [README](./README.md) file.
- Added this [CHANGELOG](./CHANGELOG.md) file.

### Changed

- Migrated the repository to GitHub and modified the AppVeyor build to publish releases as GitHub releases due to the [Bintray sunset](https://jfrog.com/blog/into-the-sunset-bintray-jcenter-gocenter-and-chartcenter/).

## [0.1.2] - 2021-01-27

### Added

- An experimental mechanism was added for tweaking the PhysFS search path by editing entries in the `.modload` file in the game directory.
- The `mods/` folder is added to the PhysFS search path and the Java classpath by default.
- The debug build launcher accepts a new option `--heap-size` and a new flag `--gc-logging` - see the usage text for details.
- The user can choose to skip repacking SFS files in the installer if they have done it before.
- BinTray publishing was set up.
- License info was added to all cargo build manifests.

### Changed

- The SFS database was updated with many more entries, enabling more of the original game files to be repacked into ZIP - thanks to carsmaster.

### Removed

- The debug build launcher no longer accepts the `--transform-classes` flag since this transformation step is now done by the installer.

### Fixed

- The launcher uses the `PhysFSLoader.loader` static method to get an instance of the `PhysFSLoader` on startup, ensuring that there is only one `PhysFSLoader` instance.

## [0.1.1] - 2021-01-25

### Added

- The license text is now shown in the installer

### Fixed

- `PhysFSInputStream` now now throws `FileNotFoundException` rather than its own `PhysFSException`, ensuring that original game code which catches `FileNotFoundException` or `IOException` continues to work as before.

## [0.1.0] - 2021-01-25

### Added
- Initial project release

[Unreleased]: https://github.com/DavidGregory084/OpenIL2/compare/0.1.3..HEAD
[0.1.3]: https://github.com/DavidGregory084/OpenIL2/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/DavidGregory084/OpenIL2/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/DavidGregory084/OpenIL2/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/DavidGregory084/OpenIL2/tree/0.1.0