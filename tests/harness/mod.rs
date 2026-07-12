use std::{ops::ControlFlow, path::PathBuf};

use jsonpath_rust::JsonPath;
use pretty_assertions::assert_eq;

use crate::utils::{GeneratedAndMigrated, generate_and_migrate_to, needs_toolchain};

/// A builder that configures and runs a migration test.
///
/// The Rust file used to generate the Rustdoc JSON is assumed to be stored at `tests/vXX/vXX.rs`,
/// where `XX` is the current format version passed to [`MigrationTest::new()`].
pub(crate) struct MigrationTest {
    original_format_version: u32,
    migrated_format_version: u32,
    path: PathBuf,
    queries: Vec<Query>,
}

impl MigrationTest {
    /// Creates a new migration test.
    pub(crate) fn new(original_format_version: u32, migrated_format_version: u32) -> Self {
        Self {
            original_format_version,
            migrated_format_version,
            path: PathBuf::from_iter([
                "tests",
                "migrations",
                &format!("v{original_format_version}.rs"),
            ]),
            queries: Vec::with_capacity(1),
        }
    }

    /// Adds a new query test.
    ///
    /// `query` is the [`JsonPath`] querying both the original and migrated Rustdoc JSON.
    /// `current_expected` is the expected query result on the original JSON, and `up_expected` is
    /// the same for the migrated JSON.
    pub(crate) fn query(
        mut self,
        query: &'static str,
        original_expected: serde_json::Value,
        migrated_expected: serde_json::Value,
    ) -> Self {
        self.queries.push(Query {
            query,
            original_expected,
            migrated_expected,
        });
        self
    }

    /// Executes this migration test.
    pub(crate) fn test(self) {
        let ControlFlow::Continue(()) = needs_toolchain(self.original_format_version) else {
            return;
        };

        let ControlFlow::Continue(()) = needs_toolchain(self.migrated_format_version) else {
            return;
        };

        let GeneratedAndMigrated {
            original_json,
            new_json,
            migrated_json,
        } = generate_and_migrate_to(
            self.path,
            self.original_format_version,
            self.migrated_format_version,
        );

        for Query {
            query,
            original_expected,
            migrated_expected,
        } in self.queries
        {
            let original_results = original_json.query_with_path(query).unwrap();
            let new_results = new_json.query_with_path(query).unwrap();
            let migrated_results = migrated_json.query_with_path(query).unwrap();

            for original_result in original_results {
                assert_eq!(
                    original_result.clone().val(),
                    &original_expected,
                    "original result does not match expected value, query {} at path {}",
                    query,
                    original_result.path()
                );
            }

            for new_result in new_results {
                assert_eq!(
                    new_result.clone().val(),
                    &migrated_expected,
                    "new result does not match expected value, query {} at path {}",
                    query,
                    new_result.path()
                );
            }

            for migrated_result in migrated_results {
                assert_eq!(
                    migrated_result.clone().val(),
                    &migrated_expected,
                    "migrated result does not match expected value, query {} at path {}",
                    query,
                    migrated_result.path()
                );
            }
        }
    }
}

/// A single JSONPath query with expected current and up results.
struct Query {
    query: &'static str,
    original_expected: serde_json::Value,
    migrated_expected: serde_json::Value,
}
