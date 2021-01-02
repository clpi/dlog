use crate::{
    models::{
        action::Action,
    },
    cmd::Cmd
};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

pub enum RelCmd {
    FactFact(String),
    FactItem(String),
    FactRecord(String),
    ItemRecord(String),
    ItemItem(String),
    RecordRecord(String),
}

impl Default for RelCmd {
    fn default() -> Self {
        Self::Help
    }
}


impl Cmd for RelCmd {

    fn name() -> &'static str { "relation" }
    fn about() -> &'static str { "The relation cmd" }
    fn long_about() -> &'static str { "The relation cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("with-attribute")
                .about("Filter by attribute")
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            clap::App::new("new"),
            Self::help_cmd(),
        ]
    }

    fn run(&self) {
        println!("{}", format!("Running rel cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("
        ").color(Color::BrightRed);
        println!("> {}", help)
    }

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("rel-help")
            .about("Prints help command for relation")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
    }

}
