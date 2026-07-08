//! The [`MigrateUp`] trait.

use crate::reporter::Reporter;

/// Migrates a `rustdoc_types` type from the current version to the next version.
pub trait MigrateUp {
    /// The new type the current type gets migrated to.
    type Up;

    /// Migrates `self` to the new version.
    fn migrate_up(self, reporter: &mut Reporter) -> Self::Up;
}
