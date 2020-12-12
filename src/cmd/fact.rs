use clap::{ArgMatches, FromArgMatches};
use super::Cmd;

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
                clap::Arg::new("help")
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
