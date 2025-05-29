//! The [`MigrateUp`] trait.

/// Migrates a `rustdoc_types` type from the current version to the next version.
pub trait MigrateUp {
    /// The new type the current type gets migrated to.
    type Up;

    /// Migrates `self` to the new version.
    fn migrate_up(self) -> Self::Up;
}

/// Represents the `Crate` type from `rustdoc_types`.
pub trait Crate {
    /// The format version of this `Crate`.
    const FORMAT_VERSION: u32;
}

impl Crate for rustdoc_types_41::Crate {
    const FORMAT_VERSION: u32 = 41;
}

impl Crate for rustdoc_types_42::Crate {
    const FORMAT_VERSION: u32 = 42;
}

impl Crate for rustdoc_types_43::Crate {
    const FORMAT_VERSION: u32 = 43;
}

impl Crate for rustdoc_types_44::Crate {
    const FORMAT_VERSION: u32 = 44;
}

impl Crate for rustdoc_types_45::Crate {
    const FORMAT_VERSION: u32 = 45;
}
