use std::{ops::ControlFlow, path::PathBuf};

use jsonpath_rust::{JsonPath, query::QueryRef};
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
    queries: Vec<QueryTest>,
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
                &format!("v{original_format_version}_to_v{migrated_format_version}.rs"),
            ]),
            queries: Vec::with_capacity(1),
        }
    }

    /// Adds a new query test.
    ///
    /// `query` is the [`JsonPath`] querying both the original and migrated Rustdoc JSON.
    /// `original_expected` is the expected query result on the original JSON, and
    /// `new_and_migrated_expected` is the same for the new and migrated JSONs.
    pub(crate) fn query(
        mut self,
        query: &'static str,
        original_expected: serde_json::Value,
        new_and_migrated_expected: serde_json::Value,
    ) -> Self {
        let new_expected = new_and_migrated_expected.clone();
        let migrated_expected = new_and_migrated_expected;

        self.queries.push(QueryTest {
            query,
            original_test: Box::new(move |result| {
                assert_eq!(
                    result.clone().val(),
                    &original_expected,
                    "original result does not match expected value, query {query} at path {path}",
                    path = result.path(),
                );
            }),
            new_test: Box::new(move |result| {
                assert_eq!(
                    result.clone().val(),
                    &new_expected,
                    "new result does not match expected value, query {query} at path {path}",
                    path = result.path(),
                );
            }),
            migrated_test: Box::new(move |result| {
                assert_eq!(
                    result.clone().val(),
                    &migrated_expected,
                    "migrated result does not match expected value, query {query} at path {path}",
                    path = result.path(),
                );
            }),
        });
        self
    }

    pub(crate) fn query_custom(
        mut self,
        query: &'static str,
        original_test: impl FnMut(QueryRef<'_, serde_json::Value>) + 'static,
        new_test: impl FnMut(QueryRef<'_, serde_json::Value>) + 'static,
        migrated_test: impl FnMut(QueryRef<'_, serde_json::Value>) + 'static,
    ) -> Self {
        self.queries.push(QueryTest {
            query,
            original_test: Box::new(original_test),
            new_test: Box::new(new_test),
            migrated_test: Box::new(migrated_test),
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

        for QueryTest {
            query,
            original_test,
            new_test,
            migrated_test,
        } in self.queries
        {
            original_json
                .query_with_path(query)
                .unwrap()
                .into_iter()
                .for_each(original_test);

            new_json
                .query_with_path(query)
                .unwrap()
                .into_iter()
                .for_each(new_test);

            migrated_json
                .query_with_path(query)
                .unwrap()
                .into_iter()
                .for_each(migrated_test);
        }
    }
}

/// A single [`JsonPath`] query with tests that verify the result on the original, new, and
/// migrated Rustdoc JSON.
///
/// Succeeding tests should return, failing tests should panic. They are passed the query result as
/// input.
struct QueryTest {
    query: &'static str,
    original_test: Box<dyn FnMut(QueryRef<'_, serde_json::Value>)>,
    new_test: Box<dyn FnMut(QueryRef<'_, serde_json::Value>)>,
    migrated_test: Box<dyn FnMut(QueryRef<'_, serde_json::Value>)>,
}
