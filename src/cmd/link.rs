use super::Cmd;
use clap::{ArgMatches, FromArgMatches};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Link {
    name: String,
    val: Option<String>,
    from: String,
    to: String,
}

impl Cmd for Link {

    fn cmd() -> clap::App<'static> {
        clap::App::new("link")
            .about("links")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("name")
            ])
    }

    fn run(&self) {

    }

    fn print_help() {
        println!("Link help")
    }

}

impl FromArgMatches for Link {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Link");
        Self::default()
    }
}
