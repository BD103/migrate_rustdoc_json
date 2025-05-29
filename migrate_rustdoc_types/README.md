# Migrate `rustdoc_types`

Migrates `rustdoc` JSON from one format version to another.

## Installation

```sh
cargo install --git https://github.com/BD103/migrate_rustdoc_types migrate_rustdoc_types
```

## Quick Start

```sh
# Build `rustdoc` JSON for a crate.
cargo +nightly rustdoc -- -Zunstable-options --output-format json

# Migrate the `rustdoc` JSON to a newer format version.
migrate_rustdoc_types --input target/doc/crate_name.json --to-version 45 > migrated.json
```
