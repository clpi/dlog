use super::Cmd;
use clap::{ArgMatches, FromArgMatches};

#[derive(Debug, Clone, Default)]
pub struct Attrib {
    name: String,
    val: String,
}

impl Cmd for Attrib {

    fn cmd() -> clap::App<'static> {
        clap::App::new("attrib")
            .about("attribs")
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
        println!("Attrib help")
    }

}

impl FromArgMatches for Attrib {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Attrib");
        Self::default()
    }
}
