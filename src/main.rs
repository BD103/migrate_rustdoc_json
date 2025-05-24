use anyhow::Context;

mod args;
mod macros;
mod migrations;
mod primitives;
mod traits;
mod untyped_crate;
mod version;

fn main() -> anyhow::Result<()> {
    let args = args::parse_args()?;

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    let output = self::migrations::migrate_up(&input, args.to_version)?;

    println!("{output}");

    Ok(())
}
