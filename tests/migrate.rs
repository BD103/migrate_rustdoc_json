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

    let (_, migrated_json) = generate_and_migrate_to("tests/v42/v42.rs", 42, 43);

    let query_expected = [
        (
            "$.index[?(@.name == 'ReprRust')].attrs",
            json!(["#[attr = Repr([ReprRust])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprC')].attrs",
            json!(["#[attr = Repr([ReprC])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprPacked1')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(1 bytes))])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprPacked2')].attrs",
            json!(["#[attr = Repr([ReprPacked(Align(2 bytes))])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprCAlign8')].attrs",
            json!(["#[attr = Repr([ReprC, ReprAlign(Align(8 bytes))])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprI8')].attrs",
            json!(["#[attr = Repr([ReprInt(SignedInt(I8))])]\n"]),
        ),
        (
            "$.index[?(@.name == 'ReprUsizeC')].attrs",
            json!(["#[attr = Repr([ReprInt(UnsignedInt(Usize)), ReprC])]\n"]),
        ),
        (
            "$.index[?(@.name == 'TransparentPub')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
        ),
        (
            "$.index[?(@.name == 'TransparentPriv')].attrs",
            json!(["#[attr = Repr([ReprTransparent])]\n"]),
        ),
    ];

    for (query, expected) in query_expected {
        let query_results = migrated_json.query(query).unwrap();

        for actual in query_results {
            assert_eq!(*actual, expected);
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
