use crate::args::Args;

use anstream::eprintln;
use anstyle::{AnsiColor, Color, Style};

/// Struct for reporting information to the user.
#[derive(Default)]
pub struct Reporter {}

impl Reporter {
    /// Configures a reporter to obey the CLI arguments.
    pub fn configure(&mut self, _args: &Args) {
        // TODO: interactive and color mode
    }

    /// Tells the user that we have started migrating.
    pub fn begin_migrating(&self, original_version: u32) {
        eprintln!(
            "{blue}Migrating JSON with format version {bold}v{original_version}{bold:#}{blue:#}",
            blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
            bold = Style::new().bold(),
        );
    }

    /// Tells the user we are migrating the JSON to a specific version.
    pub fn migrating_to(&self, version: u32) {
        eprintln!(
            "\t{dim}...to{dim:#} {blue}v{version}{blue:#}",
            dim = Style::new().dimmed().italic(),
            blue = Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
                .bold()
                .italic()
        );
    }

    /// Prints the final report after the migration has succeeded.
    pub fn print_success_report(&self) {
        eprintln!(
            "{blue}Done!{blue:#} :D",
            blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        );
    }

    /// Prints the final report after the migration has failed.
    pub fn print_error_report(&self, error: anyhow::Error) {
        let style = Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)));

        eprintln!("{style}Error{style:#}: {error:?}");
    }
}
