//! **v47 stub migration.**
//!
//! Format version v47 does not exist. This module only exists to connect v46 and v48.

use std::any::Any;

pub fn migrate_up(crate_: Box<dyn Any>) -> anyhow::Result<Box<dyn Any>> {
    // No-op, as the given `Crate` is already migrated to v48.
    Ok(crate_)
}

pub fn deserialize(_json: &str) -> anyhow::Result<Box<dyn Any>> {
    Err(anyhow::anyhow!(
        "format version v47 does not exist, the input JSON is invalid"
    ))
}

pub fn serialize(_crate: Box<dyn Any>) -> anyhow::Result<String> {
    Err(anyhow::anyhow!(
        "format version v47 does not exist, please migrate to v46 or v48 instead"
    ))
}
