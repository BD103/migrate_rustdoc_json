//! The CLI arguments and their parsing logic.

use std::{convert::Infallible, env, ffi::OsStr, path::PathBuf};

use anstream::println;
use anstyle::{AnsiColor, Color, Style};

use crate::version::ToVersion;

#[derive(Debug)]
pub struct Args {
    pub input: PathBuf,
    pub to_version: ToVersion,
}

pub fn parse_args() -> anyhow::Result<Args> {
    let args: Vec<_> = env::args_os().skip(1).collect();

    if args.is_empty() {
        print_help();
        std::process::exit(0);
    }

    let mut pico_args = pico_args::Arguments::from_vec(args);

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

    println!(
        "\
Migrate Rustdoc's JSON output to newer format versions

{bold_blue}Usage:{bold_blue:#} {blue}{BIN_NAME} --input <FILE> --to-version <VERSION> [OPTIONS]{blue:#}

{bold_blue}Options:{bold_blue:#}
  {blue}-h, --help                  {blue:#}Prints the help text and exits
  {blue}-V, --version               {blue:#}Prints the version info and exits
  {blue}    --input <FILE>          {blue:#}The Rustdoc JSON to read
  {blue}    --to-version <VERSION>  {blue:#}The format version to migrate to",
        bold_blue = Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
    );
}
