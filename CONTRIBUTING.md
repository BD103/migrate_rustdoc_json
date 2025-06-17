# Contributing

## Release Checklist

1. Remove the `-dev` suffix from the version in [`Cargo.toml`](Cargo.toml) and the compatibility table in [`README.md`](README.md).
   - Make sure [`Cargo.lock`](Cargo.lock) also updates!
2. Update [`CHANGELOG.md`](CHANGELOG.md) with all relevant changes between the current and past release.
3. Commit your changes in a commit titled `chore: release vX.Y.Z`.
4. Run `cargo publish` to publish to <https://crates.io>.
5. [Publish a new Github release](https://github.com/BD103/migrate_rustdoc_json/releases/new) with the contents of the changelog for this version.
6. Change the version in [`Cargo.toml`](Cargo.toml) to the next `-dev` version.
   - Make sure [`Cargo.lock`](Cargo.lock) also updates!
7. Add a new row to the compatibility table in [`README.md`](README.md).
8. Add a new "Unreleased" section to [`CHANGELOG.md`](CHANGELOG.md).
9. Commit your changes in a commit titled `chore: post-release vX.Y.Z`.
