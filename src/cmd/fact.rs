use clap::{ArgMatches, FromArgMatches};
use super::Cmd;
use colored::{Color, Colorize};

#[derive(Default, Debug)]
pub struct Fact;

impl Cmd for Fact{

    fn run(&self) {}

    fn cmd() -> clap::App<'static> {
        clap::App::new("fact")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("NAME")
                    .about("Name of the fact to get or make")
                    .required(false)
                    .index(2),
                clap::Arg::new("help")
                    .about("Prints help for the fact command")
            ])
    }

    fn print_help() {
        println!("Fact help")
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Fact");
        Self::default()
    }
}
