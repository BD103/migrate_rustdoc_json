//! All migrations currently implemented by this crate.

use std::{any::Any, collections::HashMap, sync::LazyLock};

use anstream::eprintln;
use anstyle::{AnsiColor, Color, Style};

type MigrateUpFn = fn(crate_: Box<dyn Any>) -> anyhow::Result<Box<dyn Any>>;
type DeserializeFn = fn(&str) -> anyhow::Result<Box<dyn Any>>;
type SerializeFn = fn(crate_: Box<dyn Any>) -> anyhow::Result<String>;

type MigrationMap = LazyLock<HashMap<u32, (MigrateUpFn, DeserializeFn, SerializeFn)>>;

/// A macro that generates the `mod v*;` statements and the [`MIGRATIONS`] map.
macro_rules! declare_migrations {
    {
        $(mod $name:ident ($version:expr);)*

        #[last]
        mod $last_name:ident ($last_version:expr, $last_rustdoc_types:path);

        static $migration_map:ident: MigrationMap = {};
    } => {
        $(mod $name;)*

        #[doc = concat!("**v", $last_version, " (de)serialization.**")]
        ///
        /// Migrating past this version is not supported, so no `migrate_up()` function is
        /// provided.
        mod $last_name {
            use $last_rustdoc_types as current;

            crate::declare_serialize_deserialize!();
        }

        static $migration_map: MigrationMap = LazyLock::new(|| {
            let migrations = [
                $(
                    (
                        $version,
                        (
                            $name::migrate_up as MigrateUpFn,
                            $name::deserialize as DeserializeFn,
                            $name::serialize as SerializeFn,
                        ),
                    ),
                )*
                (
                    $last_version,
                    (
                        unimplemented_migrate_up::<$last_version> as MigrateUpFn,
                        $last_name::deserialize as DeserializeFn,
                        $last_name::serialize as SerializeFn,
                    ),
                )
            ];

            HashMap::from(migrations)
        });
    };
}

declare_migrations! {
    mod v41 (41);
    mod v42 (42);
    mod v43 (43);
    mod v44 (44);
    mod v45 (45);
    mod v46 (46);
    mod v47 (47);

    #[last]
    mod v48 (48, rustdoc_types_48);

    static MIGRATIONS: MigrationMap = { /* macro-generated */ };
}

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

    // Convert the JSON string into a untyped `Crate`.
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

        // Migrate the untyped `Crate` through all versions between the input and the desired
        // version.
        crate_ = (migrate_up)(crate_)?;
    }

    let serialize = MIGRATIONS[&to_version].2;

    // Convert the untyped `Crate` back to a JSON string.
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
