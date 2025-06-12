//! The CLI arguments and their parsing logic.

use std::{convert::Infallible, ffi::OsStr, path::PathBuf};

use crate::version::ToVersion;

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub to_version: ToVersion,
}

pub fn parse_args() -> anyhow::Result<Args> {
    let mut pico_args = pico_args::Arguments::from_env();

    if pico_args.contains(["-h", "--help"]) {
        print_help();
        std::process::exit(0);
    }

    if pico_args.contains(["-V", "--version"]) {
        print_version();
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

fn print_version() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!("{NAME} v{VERSION}");
}

fn print_help() {
    const BIN_NAME: &str = env!("CARGO_BIN_NAME");

    println!("\
Migrate Rustdoc's JSON output to newer format versions

Usage: {BIN_NAME} --input <FILE> --to-version <VERSION> [OPTIONS]

Options:
  -h, --help                  Prints the help text and exits
  -V, --version               Prints the version info and exits
      --input <FILE>          The Rustdoc JSON to read
      --to-version <VERSION>  The format version to migrate to");
}
