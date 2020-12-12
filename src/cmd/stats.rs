use clap::{ArgMatches, FromArgMatches};
use super::Cmd;
use colored::{Color, Colorize};

#[derive(Debug)]
pub enum StatsCmd {
    Help,
}

impl Default for StatsCmd {
    fn default() -> Self {
        StatsCmd::Help
    }
}

impl Cmd for StatsCmd {

    fn run(&self) {
        println!("{}", format!("Running Stats cmd...")
            .color(Color::BrightCyan))
    }

    fn cmd() -> clap::App<'static> {
        clap::App::new("stats")
            .about("stats")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
                    .about("Prints help for the stats command")
            ])
    }

    fn print_help() {
        let help = format!("Stats
        ").color(Color::BrightCyan);
        println!("> {}", help)
    }
}

impl FromArgMatches for StatsCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for StatsCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "stats" {
            Some(Self::from_arg_matches(args))
        } else {
            None
        }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}
