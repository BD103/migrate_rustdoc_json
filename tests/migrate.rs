mod utils;

use std::ops::ControlFlow;

use serde_json::Value;
use utils::{generate_and_migrate_to, need, query_both};

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
