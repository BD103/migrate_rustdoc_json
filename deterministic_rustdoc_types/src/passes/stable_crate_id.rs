use std::collections::BTreeMap;

use serde_json::{Map, Number, Value};

pub fn pass(crate_: &mut Value) {
    let Value::Object(old_external_crates) = crate_["external_crates"].take() else {
        panic!()
    };

    // A sorted list of crate names and their old IDs. This cannot be a `BTreeMap` because there
    // may be duplicate crate names.
    let mut sorted_old_ids = Vec::with_capacity(old_external_crates.len());

    for (old_id, external_crate) in old_external_crates.iter() {
        let crate_name = external_crate["name"].as_str().unwrap();
        let old_id = old_id.parse::<u32>().unwrap();
        let html_root_url = external_crate["html_root_url"].as_str();
        sorted_old_ids.push((crate_name, html_root_url, old_id));
    }

    // Stable sort by the crate names, then HTML URL. If both are the same, we sort by old ID,
    // which may hurt determinism.
    sorted_old_ids.sort_by(|a, b| {
        a.0.cmp(b.0).then_with(|| a.1.cmp(&b.1)).then_with(|| {
            eprintln!("WARNING: Found two identical crates named {}. Sorting by old ID, which may hurt determinism.", a.0);
            a.2.cmp(&b.2)
        })
    });

    // Maps old IDs to new IDs.
    let mut id_map = BTreeMap::new();

    for (i, (_, _, old_id)) in sorted_old_ids.into_iter().enumerate() {
        let new_id = i as u32 + 1;
        id_map.insert(old_id, new_id);
    }

    let mut new_external_crates = Map::with_capacity(old_external_crates.len());

    for (old_id, external_crate) in old_external_crates {
        let old_id = old_id.parse::<u32>().unwrap();

        let new_id = id_map
            .get(&old_id)
            .expect(&format!("no entry for key {old_id}"));

        new_external_crates.insert(new_id.to_string(), external_crate);
    }

    crate_["external_crates"] = Value::Object(new_external_crates);

    for (_, item) in crate_["index"].as_object_mut().unwrap() {
        let old_id = item["crate_id"].as_u64().unwrap() as u32;

        if old_id == 0 {
            continue;
        }

        let new_id = id_map[&old_id];
        item["crate_id"] = Value::Number(Number::from(new_id));
    }

    for (_, item_summary) in crate_["paths"].as_object_mut().unwrap() {
        let old_id = item_summary["crate_id"].as_u64().unwrap() as u32;

        if old_id == 0 {
            continue;
        }

        let new_id = id_map[&old_id];
        item_summary["crate_id"] = Value::Number(Number::from(new_id));
    }
}
