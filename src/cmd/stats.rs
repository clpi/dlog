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

    fn name() -> &'static str { "stats" }
    fn about() -> &'static str { "The stats cmd" }
    fn long_about() -> &'static str { "The stats cmd" }
    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("help")
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            clap::App::new("new"),
        ]
    }
    fn run(&self) {
        println!("{}", format!("Running Stats cmd...")
            .color(Color::BrightCyan))
    }

    fn print_help() {
        let help = format!("Stats
        ").color(Color::BrightCyan);
        println!("> {}", help)
    }

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("fact_help")
            .about("Prints help command for fact")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
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
