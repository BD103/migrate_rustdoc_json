//! All migrations currently implemented by this crate.

use std::{any::Any, collections::BTreeMap, sync::LazyLock};

use anstream::eprintln;
use anstyle::{AnsiColor, Color, Style};

mod v41;
mod v42;
mod v43;
mod v44;
mod v45;

type MigrateUpFn = fn(crate_: Box<dyn Any>) -> anyhow::Result<Box<dyn Any>>;
type DeserializeFn = fn(&str) -> anyhow::Result<Box<dyn Any>>;
type SerializeFn = fn(crate_: Box<dyn Any>) -> anyhow::Result<String>;

static MIGRATIONS: LazyLock<BTreeMap<u32, (MigrateUpFn, DeserializeFn, SerializeFn)>> =
    LazyLock::new(|| {
        let migrations = [
            (
                41,
                (
                    v41::migrate_up as MigrateUpFn,
                    v41::deserialize as DeserializeFn,
                    v41::serialize as SerializeFn,
                ),
            ),
            (
                42,
                (
                    v42::migrate_up as MigrateUpFn,
                    v42::deserialize as DeserializeFn,
                    v42::serialize as SerializeFn,
                ),
            ),
            (
                43,
                (
                    v43::migrate_up as MigrateUpFn,
                    v43::deserialize as DeserializeFn,
                    v43::serialize as SerializeFn,
                ),
            ),
            (
                44,
                (
                    v44::migrate_up as MigrateUpFn,
                    v44::deserialize as DeserializeFn,
                    v44::serialize as SerializeFn,
                ),
            ),
            (
                45,
                (
                    // v46 does not exist yet, so we cannot migrate past v45.
                    unimplemented_migrate_up::<45> as MigrateUpFn,
                    v45::deserialize as DeserializeFn,
                    v45::serialize as SerializeFn,
                ),
            ),
        ];

        BTreeMap::from(migrations)
    });

pub fn migrate_up(current: &str, to_version: u32) -> anyhow::Result<String> {
    let current_version = crate::version::detect_version(current)?;

    if current_version > to_version {
        return Err(anyhow::anyhow!(
            "`--input` format version {current_version} is greater than `--to-version` {to_version}"
        )
        .context("downgrading to an older format version is not supported"));
    }

    let deserialize = MIGRATIONS[&current_version].1;

    eprintln!(
        "{blue}Migrating JSON with format version {bold}v{current_version}{bold:#}{blue:#}",
        blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        bold = Style::new().bold(),
    );

    // Convert the JSON string into an `UntypedCrate`.
    let mut crate_ = (deserialize)(current)?;

    for i in current_version..to_version {
        let migrate_up = MIGRATIONS[&i].0;

        eprintln!(
            "\t{dim}...to{dim:#} {blue}v{version}{blue:#}",
            version = i + 1,
            dim = Style::new().dimmed().italic(),
            blue = Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
                .bold()
                .italic()
        );

        // Migrate the `UntypedCrate` through all versions between the input and the desired
        // version.
        crate_ = (migrate_up)(crate_)?;
    }

    let serialize = MIGRATIONS[&to_version].2;

    // Convert the `UntypedCrate` back to a JSON string.
    (serialize)(crate_)
}

/// Panics when called, displaying an error that the given migration isn't yet supported.
fn unimplemented_migrate_up<const N: usize>(_: Box<dyn Any>) -> anyhow::Result<Box<dyn Any>> {
    fn inner(n: usize) -> anyhow::Result<Box<dyn Any>> {
        Err(anyhow::anyhow!(
            "migrating from format version v{} to format version v{} is not yet supported",
            n,
            n + 1,
        ))
    }

    inner(N)
}
