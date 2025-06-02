mod utils;

use std::ops::ControlFlow;

use jsonpath_rust::JsonPath;
use serde_json::{Value, json};
use utils::{generate_and_migrate_to, needs_toolchain, query_both};

#[test]
fn v42() {
    let ControlFlow::Continue(()) = needs_toolchain(42) else {
        return;
    };

    let (source_json, migrated_json) = generate_and_migrate_to("tests/v42/v42.rs", 42, 43);

    let query_expected = [
        (
            "$.index[?(@.name == 'ReprRust')].attrs",
            json!(["#[attr = Repr([ReprRust])]\n"]),
            json!(["#[repr(Rust)]"]),
        ),
        (
            "$.index[?(@.name == 'ReprC')].attrs",
            json!(["#[attr = Repr([ReprC])]\n"]),
            json!(["#[repr(C)]"]),
        ),
        (
            "$.index[?(@.name == 'ReprPacked1')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(1 bytes))])]\n"]),
            json!(["#[repr(packed(1))]"]),
        ),
        (
            "$.index[?(@.name == 'ReprPacked2')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(2 bytes))])]\n"]),
            json!(["#[repr(packed(2))]"]),
        ),
        (
            "$.index[?(@.name == 'ReprCAlign8')].attrs",
            json!(["#[attr = Repr([ReprC, ReprAlign(Align(8 bytes))])]\n"]),
            json!(["#[repr(C, align(8))]"]),
        ),
        (
            "$.index[?(@.name == 'ReprI8')].attrs",
            json!(["#[attr = Repr([ReprInt(SignedInt(I8))])]\n"]),
            json!(["#[repr(i8)]"]),
        ),
        (
            "$.index[?(@.name == 'ReprUsizeC')].attrs",
            json!(["#[attr = Repr([ReprInt(UnsignedInt(Usize)), ReprC])]\n"]),
            json!(["#[repr(usize, C)]"]),
        ),
        (
            "$.index[?(@.name == 'TransparentPub')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
            json!(["#[repr(transparent)]"]),
        ),
        (
            "$.index[?(@.name == 'TransparentPriv')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
            // Although `rustdoc` would hide this `#[repr(transparent)]` if the JSON was built in
            // v43, the migration doesn't yet have the logic to detect this.
            json!(["#[repr(transparent)]"]),
        ),
    ];

    for (query, source_expected, migrated_expected) in query_expected {
        let source_results = source_json.query(query).unwrap();
        let migrated_results = migrated_json.query(query).unwrap();

        for actual in source_results {
            assert_eq!(*actual, source_expected);
        }

        for actual in migrated_results {
            assert_eq!(*actual, migrated_expected);
        }
    }
}

#[test]
fn v43() {
    let ControlFlow::Continue(()) = needs_toolchain(43) else {
        return;
    };

    let (_, migrated_json) = generate_and_migrate_to("tests/v43/v43.rs", 43, 44);

    let expected = json!({
        "triple": "",
        "target_features": [],
    });

    assert_eq!(migrated_json["target"], expected);
}

#[test]
fn v44() {
    let ControlFlow::Continue(()) = needs_toolchain(44) else {
        return;
    };

    let (source_json, migrated_json) = generate_and_migrate_to("tests/v44/v44.rs", 44, 45);

    let query = "$.index[*].span['begin', 'end']";

    for (_, (source_result, migrated_result)) in query_both(&source_json, &migrated_json, query) {
        let expected = {
            let mut source_result = source_result.unwrap().clone();
            source_result[1] = Value::Number((source_result[1].as_u64().unwrap() + 1).into());
            source_result
        };

        assert_eq!(*migrated_result.unwrap(), expected);
    }
}
