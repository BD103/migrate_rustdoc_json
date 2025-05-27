use serde_json::Value;

pub fn pass(value: &mut Value) {
    let Some(target) = value.get_mut("target") else {
        return;
    };

    // Make `triple` an empty string.
    if let Some(triple) = target.get_mut("triple") {
        *triple = Value::String(String::new());
    }

    // Make `target_features` an empty array.
    if let Some(target_features) = target.get_mut("target_features") {
        *target_features = Value::Array(Vec::new());
    }
}
