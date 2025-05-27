//! The CLI arguments and their parsing logic.

use std::{convert::Infallible, ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
}

pub fn parse_args() -> anyhow::Result<Args> {
    let mut pico_args = pico_args::Arguments::from_env();

    let args = Args {
        input: pico_args.value_from_os_str("--input", |s| -> Result<PathBuf, Infallible> {
            Ok(s.into())
        })?,
    };

    let remaining = pico_args.finish();

    anyhow::ensure!(
        remaining.is_empty(),
        "unsupported arguments were passed: {}",
        remaining.join(OsStr::new(", ")).to_string_lossy()
    );

    Ok(args)
}
