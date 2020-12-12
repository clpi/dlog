use clap::{ArgMatches, FromArgMatches};
use super::Cmd;

#[derive(Default, Debug)]
pub struct Record {
    name: String,
}

impl Cmd for Record{

    fn cmd() -> clap::App<'static> {
        clap::App::new("record")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])

    }

    fn run(&self) {

    }

    fn print_help() {
        println!("Record help")
    }
}

impl FromArgMatches for Record {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Record");
        Self::default()
    }
}
