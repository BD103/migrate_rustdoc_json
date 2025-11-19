# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.4.0 - 2025-11-18

- **All Changes**: [`v0.3.0...v0.4.0`](https://github.com/BD103/migrate_rustdoc_json/compare/v0.3.0...v0.4.0)
- **Format Versions**: v41..=v50

### Added

- Format versions v48, v49, and v50 and now supported ([`1b6c5e7`](https://github.com/BD103/migrate_rustdoc_json/commit/1b6c5e765c0ef8fa6faa71c22d3003f5d9f9efb3), [`2650a40`](https://github.com/BD103/migrate_rustdoc_json/commit/2650a40c93d73c0b344e90cfa704daad20b5a179), [`b1a4c33`](https://github.com/BD103/migrate_rustdoc_json/commit/b1a4c335bfc1f017a368fe320a1cc984bc2ecc3f))
    - Format version v47 does not exist. `migrate_rustdoc_json` will skip it and migrate from v46 directly to v48. If you try to migrate JSON with a format version of 47, or specify `--to-version 47`, the CLI will exit with an error.

## v0.3.0 - 2025-06-17

- **All Changes**: [`v0.2.0...v0.3.0`](https://github.com/BD103/migrate_rustdoc_json/compare/v0.2.0...v0.3.0)
- **Format Versions**: v41..=v46

### Added

- You can now pass `--help` to view a list of CLI options supported by `migrate_rustdoc_json` ([`16d5973`](https://github.com/BD103/migrate_rustdoc_json/commit/16d5973d50a81eb9114fe814a96333ca7518e52b))
    - The help screen will also be printed if `migrate_rustdoc_json` is run with no arguments.
- Format version v46 is now supported ([`d3d2d46`](https://github.com/BD103/migrate_rustdoc_json/commit/d3d2d468daac2f00a6a8d9e703e38370d88827ba))

## v0.2.0 - 2025-06-11

- **All Changes**: [`v0.1.0...v0.2.0`](https://github.com/BD103/migrate_rustdoc_json/compare/v0.1.0...v0.2.0)
- **Format Versions**: v41..=v45

### Added

- You can now pass `--to-version latest` to migrate JSON to the latest supported format version ([`d080cb3`](https://github.com/BD103/migrate_rustdoc_json/commit/d080cb38d20eb8e22e44d83e0e498b9e867e98f3))
- Pass `--version` or `-V` to `migrate_rustdoc_types` to print the version ([`452fe1c`](https://github.com/BD103/migrate_rustdoc_json/commit/452fe1c332b0a43b77340fa6cc70525c1bdb4e01))

### Changed

- The installation instructions now recommend `--locked` when running `cargo install` ([`083c07e`](https://github.com/BD103/migrate_rustdoc_json/commit/083c07e5a7ee635f31f9e20d5823a9e5b89dd10f))

## v0.1.0 - 2025-06-03

- **All Changes**: [`v0.1.0`](https://github.com/BD103/migrate_rustdoc_json/commits/v0.1.0)
- **Format Versions**: v41..=v45

### Added

This was the first release! Nothing has changed yet :)
