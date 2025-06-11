//! The CLI arguments and their parsing logic.

use std::{convert::Infallible, ffi::OsStr, path::PathBuf};

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub to_version: u32,
}

pub fn parse_args() -> anyhow::Result<Args> {
    let mut pico_args = pico_args::Arguments::from_env();

    if pico_args.contains(["-V", "--version"]) {
        const NAME: &str = env!("CARGO_PKG_NAME");
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        println!("{NAME} v{VERSION}");

        std::process::exit(0);
    }

    let args = Args {
        input: pico_args.value_from_os_str("--input", |s| -> Result<PathBuf, Infallible> {
            Ok(s.into())
        })?,
        to_version: pico_args.value_from_str("--to-version")?,
    };

    let remaining = pico_args.finish();

    anyhow::ensure!(
        remaining.is_empty(),
        "unsupported arguments were passed: {}",
        remaining.join(OsStr::new(", ")).to_string_lossy()
    );

    Ok(args)
}
