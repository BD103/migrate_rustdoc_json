use anyhow::Context;
use serde_json::Value;

mod args;

fn main() -> anyhow::Result<()> {
    let args = self::args::parse_args()?;

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    // `Value` uses a `BTreeMap` that sorts all keys on creation.
    let value: Value = serde_json::from_str(&input)?;

    let out = serde_json::to_string(&value)?;

    println!("{out}");

    Ok(())
}
