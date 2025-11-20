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
        #[first]
        mod $first_name:ident ($first_version:expr);

        $(mod $name:ident ($version:expr);)*

        #[last]
        mod $last_name:ident ($last_version:expr, $last_rustdoc_types:path);

        static MIGRATIONS: MigrationMap = {};

        pub const MINIMUM_VERSION: u32 = {};
        pub const MAXIMUM_VERSION: u32 = {};
    } => {
        mod $first_name;
        $(mod $name;)*

        #[doc = concat!("**v", $last_version, " (de)serialization.**")]
        ///
        /// Migrating past this version is not supported, so the `migrate_up()` function is a stub
        /// that immediately errors.
        mod $last_name {
            use std::any::Any;

            use $last_rustdoc_types as current;

            crate::declare_serialize_deserialize!();

            /// Immediately returns an error that the given migration isn't yet supported.
            pub fn migrate_up(_crate: Box<dyn Any>) -> anyhow::Result<Box<dyn Any>> {
                let current_version: u32 = $last_version;

                Err(anyhow::anyhow!(
                    "migrating from format version v{} to format version v{} is not yet supported",
                    current_version,
                    current_version + 1,
                ))
            }
        }

        static MIGRATIONS: MigrationMap = LazyLock::new(|| {
            let migrations = [
                (
                    $first_version,
                    (
                        $first_name::migrate_up as MigrateUpFn,
                        $first_name::deserialize as DeserializeFn,
                        $first_name::serialize as SerializeFn,
                    ),
                ),
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
                        $last_name::migrate_up as MigrateUpFn,
                        $last_name::deserialize as DeserializeFn,
                        $last_name::serialize as SerializeFn,
                    ),
                )
            ];

            HashMap::from(migrations)
        });

        pub const MINIMUM_VERSION: u32 = $first_version;
        pub const MAXIMUM_VERSION: u32 = $last_version;
    };
}

declare_migrations! {
    #[first]
    mod v41 (41);

    mod v42 (42);
    mod v43 (43);
    mod v44 (44);
    mod v45 (45);
    mod v46 (46);
    mod v47 (47);
    mod v48 (48);
    mod v49 (49);

    #[last]
    mod v50 (50, rustdoc_types_50);

    static MIGRATIONS: MigrationMap = { /* macro-generated */ };

    pub const MINIMUM_VERSION: u32 = { /* macro-generated */ };
    pub const MAXIMUM_VERSION: u32 = { /* macro-generated */ };
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
