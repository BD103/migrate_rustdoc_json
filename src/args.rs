use std::{convert::Infallible, path::PathBuf};

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub to_version: u32,
}

pub fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pico_args = pico_args::Arguments::from_env();

    let args = Args {
        input: pico_args.value_from_os_str("--input", |s| -> Result<PathBuf, Infallible> {
            Ok(s.into())
        })?,
        to_version: pico_args.value_from_str("--to-version")?,
    };

    let remaining = pico_args.finish();
    assert!(remaining.is_empty());

    Ok(args)
}
