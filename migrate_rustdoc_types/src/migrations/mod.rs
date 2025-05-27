//! All migrations currently implemented by this crate.

use anstream::eprintln;
use anstyle::{AnsiColor, Color, Style};

use crate::untyped_crate::UntypedCrate;

mod v41;
mod v42;
mod v43;
mod v44;
mod v45;

type MigrateUpFn = fn(UntypedCrate) -> anyhow::Result<UntypedCrate>;
type DeserializeFn = fn(&str) -> anyhow::Result<UntypedCrate>;
type SerializeFn = fn(UntypedCrate) -> anyhow::Result<String>;

/// A function lookup table for migrating `rustdoc` JSON from one version to another.
///
/// `MIGRATIONS[0]` migrates v1 to v2, `MIGRATIONS[1]` migrates v2 to v3, etc.
static MIGRATIONS: [(MigrateUpFn, DeserializeFn, SerializeFn); 45] = [
    (
        unimplemented_migrate_up::<1>,
        unimplemented_deserialize::<1>,
        unimplemented_serialize::<1>,
    ),
    (
        unimplemented_migrate_up::<2>,
        unimplemented_deserialize::<2>,
        unimplemented_serialize::<2>,
    ),
    (
        unimplemented_migrate_up::<3>,
        unimplemented_deserialize::<3>,
        unimplemented_serialize::<3>,
    ),
    (
        unimplemented_migrate_up::<4>,
        unimplemented_deserialize::<4>,
        unimplemented_serialize::<4>,
    ),
    (
        unimplemented_migrate_up::<5>,
        unimplemented_deserialize::<5>,
        unimplemented_serialize::<5>,
    ),
    (
        unimplemented_migrate_up::<6>,
        unimplemented_deserialize::<6>,
        unimplemented_serialize::<6>,
    ),
    (
        unimplemented_migrate_up::<7>,
        unimplemented_deserialize::<7>,
        unimplemented_serialize::<7>,
    ),
    (
        unimplemented_migrate_up::<8>,
        unimplemented_deserialize::<8>,
        unimplemented_serialize::<8>,
    ),
    (
        unimplemented_migrate_up::<9>,
        unimplemented_deserialize::<9>,
        unimplemented_serialize::<9>,
    ),
    (
        unimplemented_migrate_up::<10>,
        unimplemented_deserialize::<10>,
        unimplemented_serialize::<10>,
    ),
    (
        unimplemented_migrate_up::<11>,
        unimplemented_deserialize::<11>,
        unimplemented_serialize::<11>,
    ),
    (
        unimplemented_migrate_up::<12>,
        unimplemented_deserialize::<12>,
        unimplemented_serialize::<12>,
    ),
    (
        unimplemented_migrate_up::<13>,
        unimplemented_deserialize::<13>,
        unimplemented_serialize::<13>,
    ),
    (
        unimplemented_migrate_up::<14>,
        unimplemented_deserialize::<14>,
        unimplemented_serialize::<14>,
    ),
    (
        unimplemented_migrate_up::<15>,
        unimplemented_deserialize::<15>,
        unimplemented_serialize::<15>,
    ),
    (
        unimplemented_migrate_up::<16>,
        unimplemented_deserialize::<16>,
        unimplemented_serialize::<16>,
    ),
    (
        unimplemented_migrate_up::<17>,
        unimplemented_deserialize::<17>,
        unimplemented_serialize::<17>,
    ),
    (
        unimplemented_migrate_up::<18>,
        unimplemented_deserialize::<18>,
        unimplemented_serialize::<18>,
    ),
    (
        unimplemented_migrate_up::<19>,
        unimplemented_deserialize::<19>,
        unimplemented_serialize::<19>,
    ),
    (
        unimplemented_migrate_up::<20>,
        unimplemented_deserialize::<20>,
        unimplemented_serialize::<20>,
    ),
    (
        unimplemented_migrate_up::<21>,
        unimplemented_deserialize::<21>,
        unimplemented_serialize::<21>,
    ),
    (
        unimplemented_migrate_up::<22>,
        unimplemented_deserialize::<22>,
        unimplemented_serialize::<22>,
    ),
    (
        unimplemented_migrate_up::<23>,
        unimplemented_deserialize::<23>,
        unimplemented_serialize::<23>,
    ),
    (
        unimplemented_migrate_up::<24>,
        unimplemented_deserialize::<24>,
        unimplemented_serialize::<24>,
    ),
    (
        unimplemented_migrate_up::<25>,
        unimplemented_deserialize::<25>,
        unimplemented_serialize::<25>,
    ),
    (
        unimplemented_migrate_up::<26>,
        unimplemented_deserialize::<26>,
        unimplemented_serialize::<26>,
    ),
    (
        unimplemented_migrate_up::<27>,
        unimplemented_deserialize::<27>,
        unimplemented_serialize::<27>,
    ),
    (
        unimplemented_migrate_up::<28>,
        unimplemented_deserialize::<28>,
        unimplemented_serialize::<28>,
    ),
    (
        unimplemented_migrate_up::<29>,
        unimplemented_deserialize::<29>,
        unimplemented_serialize::<29>,
    ),
    (
        unimplemented_migrate_up::<30>,
        unimplemented_deserialize::<30>,
        unimplemented_serialize::<30>,
    ),
    (
        unimplemented_migrate_up::<31>,
        unimplemented_deserialize::<31>,
        unimplemented_serialize::<31>,
    ),
    (
        unimplemented_migrate_up::<32>,
        unimplemented_deserialize::<32>,
        unimplemented_serialize::<32>,
    ),
    (
        unimplemented_migrate_up::<33>,
        unimplemented_deserialize::<33>,
        unimplemented_serialize::<33>,
    ),
    (
        unimplemented_migrate_up::<34>,
        unimplemented_deserialize::<34>,
        unimplemented_serialize::<34>,
    ),
    (
        unimplemented_migrate_up::<35>,
        unimplemented_deserialize::<35>,
        unimplemented_serialize::<35>,
    ),
    (
        unimplemented_migrate_up::<36>,
        unimplemented_deserialize::<36>,
        unimplemented_serialize::<36>,
    ),
    (
        unimplemented_migrate_up::<37>,
        unimplemented_deserialize::<37>,
        unimplemented_serialize::<37>,
    ),
    (
        unimplemented_migrate_up::<38>,
        unimplemented_deserialize::<38>,
        unimplemented_serialize::<38>,
    ),
    (
        unimplemented_migrate_up::<39>,
        unimplemented_deserialize::<39>,
        unimplemented_serialize::<39>,
    ),
    (
        unimplemented_migrate_up::<40>,
        unimplemented_deserialize::<40>,
        unimplemented_serialize::<40>,
    ),
    (v41::migrate_up, v41::deserialize, v41::serialize),
    (v42::migrate_up, v42::deserialize, v42::serialize),
    (v43::migrate_up, v43::deserialize, v43::serialize),
    (v44::migrate_up, v44::deserialize, v44::serialize),
    (
        // v46 does not exist yet, so we cannot migrate past v45.
        unimplemented_migrate_up::<45>,
        v45::deserialize,
        v45::serialize,
    ),
];

pub fn migrate_up(current: &str, to_version: u32) -> anyhow::Result<String> {
    let current_version = crate::version::detect_version(current)?;

    if current_version > to_version {
        return Err(anyhow::anyhow!(
            "`--input` format version {current_version} is greater than `--to-version` {to_version}"
        )
        .context("downgrading to an older format version is not supported"));
    }

    let deserialize = MIGRATIONS[current_version as usize - 1].1;

    {
        let blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue)));
        let bold = Style::new().bold();

        eprintln!(
            "{blue}Migrating JSON with format version {bold}v{current_version}{bold:#}{blue:#}"
        );
    }

    // Convert the JSON string into an `UntypedCrate`.
    let mut crate_ = (deserialize)(current)?;

    for i in current_version..to_version {
        let migrate_up = MIGRATIONS[i as usize - 1].0;

        {
            let dim = Style::new().dimmed().italic();
            let blue = Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
                .bold()
                .italic();

            eprintln!("\t{dim}...to{dim:#} {blue}v{}{blue:#}", i + 1);
        }

        // Migrate the `UntypedCrate` through all versions between the input and the desired
        // version.
        crate_ = (migrate_up)(crate_)?;
    }

    let serialize = MIGRATIONS[to_version as usize - 1].2;

    // Convert the `UntypedCrate` back to a JSON string.
    (serialize)(crate_)
}

/// Panics when called, displaying an error that the given migration isn't yet supported.
fn unimplemented_migrate_up<const N: usize>(_: UntypedCrate) -> anyhow::Result<UntypedCrate> {
    fn inner(n: usize) -> anyhow::Result<UntypedCrate> {
        Err(anyhow::anyhow!(
            "migrating from format version v{} to format version v{} is not yet supported",
            n,
            n + 1,
        ))
    }

    inner(N)
}

/// Errors when called, displaying an error that deserialization isn't yet supported.
fn unimplemented_deserialize<const N: usize>(_: &str) -> anyhow::Result<UntypedCrate> {
    fn inner(n: usize) -> anyhow::Result<UntypedCrate> {
        Err(anyhow::anyhow!("format version v{n} is not yet supported"))
    }

    inner(N)
}

/// Panics when called, displaying an error that serialization should be supported but isn't.
fn unimplemented_serialize<const N: usize>(_: UntypedCrate) -> anyhow::Result<String> {
    fn inner(n: usize) -> String {
        // This is unreachable because if we support migrating to this version, we should support
        // serializing it as well.
        unreachable!("unable to convert `Crate` format version v{n} to JSON");
    }

    Ok(inner(N))
}
