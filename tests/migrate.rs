mod utils;

use std::{ops::ControlFlow, path::Path};

use jsonpath_rust::JsonPath;
use serde_json::{Number, Value};
use utils::{generate_json, migrate_json, need, path_val_map, read_json};

#[test]
fn v44() {
    let ControlFlow::Continue(()) = need(44) else {
        return;
    };

    let original = generate_json(Path::new("tests/v44/v44.rs"), 44);
    let migrated = migrate_json(&original, 45);

    let original_json = read_json(&original);
    let migrated_json = read_json(&migrated);

    let query = "$.index[*].span['begin', 'end']";

    let original_spans = path_val_map(original_json.query_with_path(query).unwrap());

    for migrated_span in migrated_json.query_with_path(query).unwrap() {
        let path = migrated_span.clone().path();

        let mut original_span = original_spans.get(&path).unwrap().clone();

        if let Value::Number(ref mut col) = original_span[1] {
            *col = Number::from(col.as_u64().unwrap() + 1);
        }

        let migrated_span = migrated_span.val();

        assert_eq!(&original_span, migrated_span);
    }
}
