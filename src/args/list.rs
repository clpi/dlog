use clap::IntoApp;
use crate::cmd::Cmd;
use colored::{Color, Colorize};
use super::search::Filters;

#[derive(Default, Debug)]
pub struct List {
    filters: Filters,
}

impl List {

}

impl Cmd for List {

    fn name() -> &'static str { "list" }
    fn about() -> &'static str { "The list cmd" }
    fn long_about() -> &'static str { "The list cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            Self::help_cmd(),
        ]
    }

    fn run(&self) {
        println!("{}", format!("Running list cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("action").color(Color::BrightRed);
        println!("> {}", help)
    }

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("fact_help")
            .about("Prints help command for list")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
    }

}

impl IntoApp for List {
    fn into_app<'help>() -> clap::App<'help> {
        clap::App::new("search")
            .about("")
    }
    fn augment_clap(app: clap::App<'_>) -> clap::App<'_> {
       app.subcommand(Self::into_app())
    }
}

impl clap::FromArgMatches for List {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Self {
        List { filters: Filters::None}
    }
}
