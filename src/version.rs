//! Format-version parsing from `rustdoc` JSON.

use anyhow::Context;
use serde::Deserialize;

/// A simplified version of `rustdoc_types::Crate` that only has the `format_version` field.
///
/// This type is intended to be independent of the format version so that it may deserialize any
/// version of `rustdoc`'s JSON output.
#[derive(Deserialize)]
struct Crate {
    pub format_version: u32,
}

/// Detects the format version from `rustdoc`'s JSON output.
pub fn detect_version(json: &str) -> anyhow::Result<u32> {
    let Crate { format_version } =
        serde_json::from_str(json).context("failed to read format version of JSON")?;

    Ok(format_version)
}
