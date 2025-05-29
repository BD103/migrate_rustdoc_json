# `rustdoc_types` Utilities

This repository is a collection of CLI utilities for managing `rustdoc`'s [unstable JSON output](https://doc.rust-lang.org/stable/rustdoc/unstable-features.html#json).

- [`migrate_rustdoc_types`](migrate_rustdoc_types/README.md): Migrate `rustdoc` JSON from one format version to another.
- [`deterministic_rustdoc_types`](deterministic_rustdoc_types/README.md) (WIP 🚧): Modify `rustdoc` JSON to be deterministic, making it suitable for snapshot testing.
