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
            // Although Rustdoc would hide this `#[repr(transparent)]` if the JSON was built in
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

    for (source_result, migrated_result) in
        query_both(&source_json, &migrated_json, query).into_values()
    {
        let expected = {
            let mut source_result = source_result.unwrap().clone();
            source_result[1] = Value::Number((source_result[1].as_u64().unwrap() + 1).into());
            source_result
        };

        assert_eq!(*migrated_result.unwrap(), expected);
    }
}

#[test]
fn v45() {
    let ControlFlow::Continue(()) = needs_toolchain(45) else {
        return;
    };

    let (_, migrated_json) = generate_and_migrate_to("tests/v45/v45.rs", 45, 46);

    let pub_query = "$.index[?(@.name == 'TransparentPub')].attrs";
    let pub_expected = json!(["#[repr(transparent)]"]);

    let priv_query = "$.index[?(@.name == 'TransparentPriv')].attrs";
    let priv_expected = json!([]);

    for result in migrated_json.query(pub_query).unwrap() {
        assert_eq!(result, &pub_expected);
    }

    for result in migrated_json.query(priv_query).unwrap() {
        assert_eq!(result, &priv_expected);
    }
}

#[test]
fn v46() {
    let ControlFlow::Continue(()) = needs_toolchain(46) else {
        return;
    };

    let (source_json, migrated_json) = generate_and_migrate_to("tests/v46/v46.rs", 46, 48);

    let query_expected = [
        (
            "$.index[?(@.name == 'inline_hint')].attrs",
            json!(["#[inline]"]),
            json!(["#[attr = Inline(Hint)]"]),
        ),
        (
            "$.index[?(@.name == 'inline_always')].attrs",
            json!(["#[inline(always)]"]),
            json!(["#[attr = Inline(Always)]"]),
        ),
        (
            "$.index[?(@.name == 'inline_never')].attrs",
            json!(["#[inline(never)]"]),
            json!(["#[attr = Inline(Never)]"]),
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
fn v48() {
    let ControlFlow::Continue(()) = needs_toolchain(48) else {
        return;
    };

    let (source_json, migrated_json) = generate_and_migrate_to("tests/v48/v48.rs", 48, 49);

    let query_expected = [
        (
            "$.index[?(@.name == 'do_not_optimize')].attrs",
            json!(["#[optimize(none)]"]),
            json!(["#[attr = Optimize(DoNotOptimize)]"]),
        ),
        (
            "$.index[?(@.name == 'optimize_speed')].attrs",
            json!(["#[optimize(speed)]"]),
            json!(["#[attr = Optimize(Speed)]"]),
        ),
        (
            "$.index[?(@.name == 'optimize_size')].attrs",
            json!(["#[optimize(size)]"]),
            json!(["#[attr = Optimize(Size)]"]),
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
