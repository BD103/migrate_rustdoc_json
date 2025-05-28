use std::{collections::BTreeMap, mem};

use anyhow::Context;
use serde_json::{Map, Number, Value};

#[derive(Debug)]
struct ExternalCrates(Map<String, Value>);

impl ExternalCrates {
    pub fn take_from_crate(crate_: &mut Value) -> anyhow::Result<Self> {
        let Value::Object(ref mut reference) = crate_["external_crates"] else {
            anyhow::bail!("`external_crates` is not a map");
        };

        let external_crates = mem::replace(reference, Map::new());

        Ok(Self(external_crates))
    }
}

#[derive(Debug)]
struct CrateMap(BTreeMap<u32, u32>);

impl CrateMap {
    pub fn from_external_crates(external_crates: &ExternalCrates) -> anyhow::Result<Self> {
        let mut sorted_ids = BTreeMap::new();

        for (id, external_crate) in external_crates.0.iter() {
            let Some(name) = external_crate["name"].as_str().map(str::to_string) else {
                anyhow::bail!("could not extract `name` from `external_crates`");
            };

            let id = id
                .parse::<u32>()
                .context("could not parse crate id as a `u32`")?;

            sorted_ids.insert(name, id);
        }

        let mut crate_map = BTreeMap::new();

        for i in sorted_ids.into_values().enumerate() {
            let (new_id, old_id) = (i.0 as u32 + 1, i.1);
            crate_map.insert(old_id, new_id);
        }

        Ok(Self(crate_map))
    }

    pub fn get(&self, old_id: u32) -> Option<u32> {
        eprintln!("GET: {old_id}");
        self.0.get(&old_id).copied()
    }
}

pub fn pass(crate_: &mut Value) -> anyhow::Result<()> {
    let external_crates = ExternalCrates::take_from_crate(crate_)?;
    let crate_map = dbg!(CrateMap::from_external_crates(&external_crates)?);

    let new_external_crates: Map<String, Value> = external_crates
        .0
        .into_iter()
        .map(|(k, v)| {
            let k = k.parse::<u32>().unwrap();
            let new_k = crate_map.get(k).unwrap().to_string();
            (new_k, v)
        })
        .collect();

    crate_["external_crates"] = Value::Object(new_external_crates);

    for (_, item) in crate_["index"].as_object_mut().unwrap() {
        let new_id = crate_map.get(item["crate_id"].as_u64().unwrap() as u32).unwrap();
        item["crate_id"] = Value::Number(Number::from(new_id));
    }

    for (_, item_summary) in crate_["paths"].as_object_mut().unwrap() {
        let new_id = crate_map.get(item_summary["crate_id"].as_u64().unwrap() as u32).unwrap();
        item_summary["crate_id"] = Value::Number(Number::from(new_id));
    }

    Ok(())
}
