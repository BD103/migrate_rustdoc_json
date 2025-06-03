# Migrate Rustdoc JSON

Migrates Rustdoc JSON from one format version to another.

## Why is this useful?

There are [several tools](https://crates.io/crates/rustdoc-types/reverse_dependencies) that read Rustdoc's JSON format, the largest of which being [`cargo-semver-checks`](https://github.com/obi1kenobi/cargo-semver-checks) and [`cargo-public-api`](https://github.com/cargo-public-api/cargo-public-api).

These tools either support only a _specific_ format version of the JSON or do not support comparing JSON with _different_ format versions. This is an issue for projects that depend on [the compiler's internal APIs](https://doc.rust-lang.org/nightly/unstable-book/language-features/rustc-private.html) (`rustc` drivers) and for projects that require an older, unsupported compiler.

## Installation

You can install the latest release of `migrate_rustdoc_json` with `cargo install`:

```sh
cargo install migrate_rustdoc_json --locked
```

You can install the latest unstable version from Git:

```sh
cargo install --git https://github.com/BD103/migrate_rustdoc_json --locked
```

## Quick Start

```sh
migrate_rustdoc_json --input path/to/rustdoc.json --to-version 45 > migrated.json
```

## Usage

In order to migrate Rustdoc's JSON output, you must first build the original JSON:

```sh
# Build Rustdoc JSON for a crate.
cargo +nightly rustdoc -- -Zunstable-options --output-format json
```

You'll likely need to change `+nightly` to a specific toolchain. You may also be interested in passing `--document-hidden-items` and `--document-private-items`.

Once you've built the JSON, you can use `migrate_rustdoc_json` to migrate it to a later version:

```sh
# Migrate the Rustdoc JSON to a newer format version.
migrate_rustdoc_json --input target/doc/crate_name.json --to-version 45 > migrated.json
```

`migrate_rustdoc_json` prints the migrated JSON to `stdout`, which is why you need to pipe it to a file with `> migrated.json`. This tool only supports migrating to newer format versions (such as v43 to v45). **Migrating down (such as v46 to v42) is unsupported.**

## Compatibility

|`merge_rustdoc_json` Version|Format Version|
|-|-|
|v0.2.0-dev|v41..=v45|
|v0.1.0|v41..=v45|

This table shows which range of format versions a given release of `merge_rustdoc_json` supports. `merge_rustdoc_json` can read any JSON within the range, and can migrate it to any greater version within the range.

## History

This tool was originally created for the [Bevy Linter](https://thebevyflock.github.io/bevy_cli/linter/index.html) to assist with updating to newer versions of the Rust compiler and [`clippy_utils`](https://crates.io/crates/clippy_utils). Since neither dependency provides an internal changelog and each release requires a different Rust compiler version, `migrate_rustdoc_json` was created to allow viewing the breaking changes with `cargo-semver-checks`.

## License

`migrate_rustdoc_json` is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
