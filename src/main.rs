use std::process::ExitCode;

use anstream::println;
use anyhow::Context;

use self::reporter::Reporter;

mod args;
mod macros;
mod migrations;
mod primitives;
mod reporter;
mod traits;
mod version;

/// The main entrypoint with a custom error handler.
///
/// For the program logic, see [`migrate_rustdoc_json()`].
fn main() -> ExitCode {
    let mut reporter = Reporter::default();

    match migrate_rustdoc_json(&mut reporter) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            reporter.print_error_report(error);

            ExitCode::FAILURE
        }
    }
}

/// The main program logic.
fn migrate_rustdoc_json(reporter: &mut Reporter) -> anyhow::Result<()> {
    let args = args::parse_args()?;

    reporter.configure(&args);

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    let output = self::migrations::migrate_up(&input, args.to_version.format_version(), reporter)?;

    reporter.print_success_report();

    println!("{output}");

    Ok(())
}
