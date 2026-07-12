use std::{ops::ControlFlow, path::PathBuf};

use jsonpath_rust::JsonPath;
use pretty_assertions::assert_eq;

use crate::utils::{generate_and_migrate_to, needs_toolchain};

/// A builder that configures and runs a migration test.
///
/// The Rust file used to generate the Rustdoc JSON is assumed to be stored at `tests/vXX/vXX.rs`,
/// where `XX` is the current format version passed to [`MigrationTest::new()`].
pub(crate) struct MigrationTest {
    current: u32,
    up: u32,
    path: PathBuf,
    queries: Vec<Query>,
}

impl MigrationTest {
    /// Creates a new migration test.
    ///
    /// `current` is the format version of the original Rustdoc JSON, and `up` is the format
    /// version that JSON will be migrated to using `migrate_rustdoc_json`.
    pub(crate) fn new(current: u32, up: u32) -> Self {
        Self {
            current,
            up,
            path: PathBuf::from_iter(["tests", &format!("v{current}"), &format!("v{current}.rs")]),
            queries: Vec::with_capacity(1),
        }
    }

    /// Adds a new query test.
    ///
    /// `query` is the `JSONPath` querying both the original and migrated Rustdoc JSON.
    /// `current_expected` is the expected query result on the original JSON, and `up_expected` is
    /// the same for the migrated JSON.
    pub(crate) fn query(
        mut self,
        query: &'static str,
        current_expected: serde_json::Value,
        up_expected: serde_json::Value,
    ) -> Self {
        self.queries.push(Query {
            query,
            current_expected,
            up_expected,
        });
        self
    }

    /// Executes this migration test.
    pub(crate) fn test(self) {
        let ControlFlow::Continue(()) = needs_toolchain(self.current) else {
            return;
        };

        let (current_json, up_json) = generate_and_migrate_to(self.path, self.current, self.up);

        for Query {
            query,
            current_expected,
            up_expected,
        } in self.queries
        {
            let current_results = current_json.query_with_path(query).unwrap();
            let up_results = up_json.query_with_path(query).unwrap();

            for current_result in current_results {
                assert_eq!(
                    current_result.clone().val(),
                    &current_expected,
                    "current result does not match expected value, query {} at path {}",
                    query,
                    current_result.path()
                );
            }

            for up_result in up_results {
                assert_eq!(
                    up_result.clone().val(),
                    &up_expected,
                    "up result does not match expected value, query {} at path {}",
                    query,
                    up_result.path()
                );
            }
        }
    }
}

/// A single JSONPath query with expected current and up results.
struct Query {
    query: &'static str,
    current_expected: serde_json::Value,
    up_expected: serde_json::Value,
}
