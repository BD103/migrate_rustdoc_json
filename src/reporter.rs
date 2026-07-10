use crate::args::Args;

use anstream::eprintln;
use anstyle::{AnsiColor, Color, Style};

/// Struct for reporting information to the user.
#[derive(Default)]
pub struct Reporter {
    currently_migrating_to: u32,
    caveats: Vec<Caveat>,
}

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
    pub fn migrating_to(&mut self, version: u32) {
        self.currently_migrating_to = version;

        eprintln!(
            "\t{dim}...to{dim:#} {blue}v{version}{blue:#}",
            dim = Style::new().dimmed().italic(),
            blue = Style::new()
                .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
                .bold()
                .italic()
        );
    }

    /// Reports a caveat, that there was an imperfect migration that may require user intervention.
    pub fn caveat(&mut self, message: String) {
        self.caveats.push(Caveat {
            message,
            while_migrating_to: self.currently_migrating_to,
        });
    }

    /// Prints the final report after the migration has succeeded.
    pub fn print_success_report(&self) {
        eprintln!(
            "{blue}Done!{blue:#} :D",
            blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))),
        );

        if !self.caveats.is_empty() {
            eprintln!(
                "\n{yellow}Caveats:{yellow:#}",
                yellow = Style::new()
                    .fg_color(Some(Color::Ansi(AnsiColor::Yellow)))
                    .bold()
            );

            for caveat in self.caveats.iter() {
                eprintln!(
                    "\t- {msg} {dim}(while migrating to v{migrating_to}){dim:#}",
                    msg = caveat.message,
                    migrating_to = caveat.while_migrating_to,
                    dim = Style::new().dimmed().italic(),
                );
            }
        }
    }

    /// Prints the final report after the migration has failed.
    pub fn print_error_report(&self, error: anyhow::Error) {
        let style = Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)));

        eprintln!("{style}Error{style:#}: {error:?}");
    }
}

/// A caveat, used to note imperfect migrations that may require user intervention.
struct Caveat {
    message: String,
    while_migrating_to: u32,
}
