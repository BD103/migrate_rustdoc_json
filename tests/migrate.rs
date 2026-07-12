mod harness;
mod utils;

use std::ops::ControlFlow;

use serde_json::{Value, json};
use utils::{generate_and_migrate_to, needs_toolchain, query_both};

use self::{harness::MigrationTest, utils::GeneratedAndMigrated};

#[test]
fn v42_to_v43() {
    MigrationTest::new(42, 43)
        .query(
            "$.index[?(@.name == 'ReprRust')].attrs",
            json!(["#[attr = Repr([ReprRust])]\n"]),
            json!(["#[repr(Rust)]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprC')].attrs",
            json!(["#[attr = Repr([ReprC])]\n"]),
            json!(["#[repr(C)]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprPacked1')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(1 bytes))])]\n"]),
            json!(["#[repr(packed(1))]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprPacked2')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(2 bytes))])]\n"]),
            json!(["#[repr(packed(2))]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprCAlign8')].attrs",
            json!(["#[attr = Repr([ReprC, ReprAlign(Align(8 bytes))])]\n"]),
            json!(["#[repr(C, align(8))]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprI8')].attrs",
            json!(["#[attr = Repr([ReprInt(SignedInt(I8))])]\n"]),
            json!(["#[repr(i8)]"]),
        )
        .query(
            "$.index[?(@.name == 'ReprUsizeC')].attrs",
            json!(["#[attr = Repr([ReprInt(UnsignedInt(Usize)), ReprC])]\n"]),
            json!(["#[repr(usize, C)]"]),
        )
        .query(
            "$.index[?(@.name == 'TransparentPub')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
            json!(["#[repr(transparent)]"]),
        )
        .query(
            "$.index[?(@.name == 'TransparentPriv')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
            // Although Rustdoc would hide this `#[repr(transparent)]` if the JSON was built in
            // v43, the migration doesn't yet have the logic to detect this.
            json!(["#[repr(transparent)]"]),
        )
        .test();
}

#[test]
fn v43_to_v44() {
    MigrationTest::new(43, 44)
        .custom(|original_json, new_json, migrated_json| {
            // Original tests.
            assert!(
                original_json.get("target").is_none(),
                "original should not have `target` field: {}",
                original_json["target"]
            );

            // New tests.
            assert!(
                new_json["target"]["triple"].is_string(),
                "new `target.triple` is not a string: {}",
                new_json["target"]["triple"]
            );
            assert!(
                new_json["target"]["target_features"].is_array(),
                "new `target.target_features` is not an array: {}",
                new_json["target"]["target_features"]
            );

            // Migrated tests.
            assert_eq!(migrated_json["target"]["triple"], "");
            assert_eq!(migrated_json["target"]["target_features"], json!([]));
        })
        .test();
}

#[test]
fn v44_to_v45() {
    let ControlFlow::Continue(()) = needs_toolchain(44) else {
        return;
    };

    let GeneratedAndMigrated {
        original_json,
        migrated_json,
        ..
    } = generate_and_migrate_to("tests/migrations/v44_to_v45.rs", 44, 45);

    let query = "$.index[*].span['begin', 'end']";

    for (original_result, migrated_result) in
        query_both(&original_json, &migrated_json, query).into_values()
    {
        let expected = {
            let mut original_result = original_result.unwrap().clone();
            original_result[1] = Value::Number((original_result[1].as_u64().unwrap() + 1).into());
            original_result
        };

        assert_eq!(*migrated_result.unwrap(), expected);
    }
}

#[test]
fn v45_to_v46() {
    MigrationTest::new(45, 46)
        .query(
            "$.index[?(@.name == 'TransparentPub')].attrs",
            json!(["#[repr(transparent)]"]),
            json!(["#[repr(transparent)]"]),
        )
        .query(
            "$.index[?(@.name == 'TransparentPriv')].attrs",
            json!([]),
            json!([]),
        )
        .test();
}

#[test]
fn v46_to_v48() {
    MigrationTest::new(46, 48)
        .query(
            "$.index[?(@.name == 'inline_hint')].attrs",
            json!(["#[inline]"]),
            json!(["#[attr = Inline(Hint)]"]),
        )
        .query(
            "$.index[?(@.name == 'inline_always')].attrs",
            json!(["#[inline(always)]"]),
            json!(["#[attr = Inline(Always)]"]),
        )
        .query(
            "$.index[?(@.name == 'inline_never')].attrs",
            json!(["#[inline(never)]"]),
            json!(["#[attr = Inline(Never)]"]),
        )
        .test();
}

#[test]
fn v48_to_v49() {
    MigrationTest::new(48, 49)
        .query(
            "$.index[?(@.name == 'do_not_optimize')].attrs",
            json!(["#[optimize(none)]"]),
            json!(["#[attr = Optimize(DoNotOptimize)]"]),
        )
        .query(
            "$.index[?(@.name == 'optimize_speed')].attrs",
            json!(["#[optimize(speed)]"]),
            json!(["#[attr = Optimize(Speed)]"]),
        )
        .query(
            "$.index[?(@.name == 'optimize_size')].attrs",
            json!(["#[optimize(size)]"]),
            json!(["#[attr = Optimize(Size)]"]),
        )
        .test();
}

#[test]
fn v49_to_v51() {
    // No nightly version can generate v50, so we migrate to v51 instead.
    MigrationTest::new(49, 51)
        .query(
            "$.index[?(@.name == 'cold')].attrs",
            json!(["#[cold]"]),
            json!(["#[attr = Cold]"]),
        )
        .test();
}

#[test]
fn v51_to_v52() {
    MigrationTest::new(51, 52)
        .query(
            "$.index[?(@.name == 'without_message')].attrs",
            json!(["#[must_use]"]),
            json!(["#[attr = MustUse]"]),
        )
        .query(
            "$.index[?(@.name == 'with_message')].attrs",
            json!(["#[must_use = \"custom message\"]"]),
            json!(["#[attr = MustUse {reason: \"custom message\"}]"]),
        )
        .test();
}
