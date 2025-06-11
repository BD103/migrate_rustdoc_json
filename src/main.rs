use std::process::ExitCode;

use anstream::{eprintln, println};
use anstyle::{AnsiColor, Color, Style};
use anyhow::Context;

mod args;
mod macros;
mod migrations;
mod primitives;
mod traits;
mod version;

/// The main entrypoint with a custom error handler.
///
/// For the program logic, see [`migrate_rustdoc_json()`].
fn main() -> ExitCode {
    match migrate_rustdoc_json() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let style = Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)));

            eprintln!("{style}Error{style:#}: {error:?}");

            ExitCode::FAILURE
        }
    }
}

/// The main program logic.
fn migrate_rustdoc_json() -> anyhow::Result<()> {
    let args = args::parse_args()?;

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    let output = self::migrations::migrate_up(&input, args.to_version)?;

    eprintln!(
        "{blue}Done!{blue:#} :D",
        blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
    );

    println!("{output}");

    Ok(())
}
