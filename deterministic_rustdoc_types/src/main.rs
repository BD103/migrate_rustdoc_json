use anyhow::Context;
use serde_json::Value;

mod args;
mod passes;

fn main() -> anyhow::Result<()> {
    let args = self::args::parse_args()?;

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    // `Value` uses a `BTreeMap` that sorts all keys on creation.
    let mut value: Value = serde_json::from_str(&input)?;

    if !is_supported_version(&value) {
        panic!("unsupported version");
    }

    self::passes::remove_target::pass(&mut value);
    self::passes::stable_crate_id::pass(&mut value)?;

    let out = serde_json::to_string(&value)?;

    println!("{out}");

    Ok(())
}

fn is_supported_version(value: &Value) -> bool {
    let Some(format_version) = value
        .get("format_version")
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
    else {
        return false;
    };

    let supported_versions = 45..=45;

    supported_versions.contains(&format_version)
}
