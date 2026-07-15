# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- **All Changes**: [`v0.4.0...main`](https://github.com/BD103/migrate_rustdoc_json/compare/v0.4.0...main)
- **Format Versions**: v41..=v53

### Added

- Format versions v51, v52, and v53 are now supported ([`a732263`](https://github.com/BD103/migrate_rustdoc_json/commit/a732263015c701ecc64e393c659c86e9d2e38154), [`e00a35f`](https://github.com/BD103/migrate_rustdoc_json/commit/e00a35f728dce6556e23c6c564957751c93900cb), [`4db85b5`](https://github.com/BD103/migrate_rustdoc_json/commit/4db85b5cbecf2bc7b91bdaf88da7d437a78e034e))
    - Note that while v50 is a valid format version, no nightly Rust toolchain generates it.
- Caveats are now reported when `migrate_rustdoc_json` is unable to perfectly migrate its input ([`5d53f69`](https://github.com/BD103/migrate_rustdoc_json/commit/5d53f695db7036f786dfc1cb02e8736b37c0a940))

### Fixed

- Corrected v0.4.0 version in compatibility table ([`c0de24b`](https://github.com/BD103/migrate_rustdoc_json/commit/c0de24b1132f2a404eae9133685c88bde02f077e))

## v0.4.0 - 2025-11-18

- **All Changes**: [`v0.3.0...v0.4.0`](https://github.com/BD103/migrate_rustdoc_json/compare/v0.3.0...v0.4.0)
- **Format Versions**: v41..=v50

### Added

- Format versions v48, v49, and v50 are now supported ([`1b6c5e7`](https://github.com/BD103/migrate_rustdoc_json/commit/1b6c5e765c0ef8fa6faa71c22d3003f5d9f9efb3), [`2650a40`](https://github.com/BD103/migrate_rustdoc_json/commit/2650a40c93d73c0b344e90cfa704daad20b5a179), [`b1a4c33`](https://github.com/BD103/migrate_rustdoc_json/commit/b1a4c335bfc1f017a368fe320a1cc984bc2ecc3f))
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
