//! The [`MigrateUp`] trait.

/// Migrates a `rustdoc_types` type from the current version to the next version.
pub trait MigrateUp {
    /// The new type the current type gets migrated to.
    type Up;

    /// Migrates `self` to the new version.
    fn migrate_up(self) -> Self::Up;
}
