mod args;

fn main() -> anyhow::Result<()> {
    let args = self::args::parse_args()?;

    Ok(())
}
