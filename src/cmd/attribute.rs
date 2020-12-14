use super::Cmd;
use serde::{Serialize, Deserialize};
use clap::{Clap, ArgMatches, FromArgMatches};
use colored::{Colorize, Color};

#[derive(Debug)]
pub enum AttribCmd {
    New(Option<Attrib>),
    Help,
    List,
}

impl Default for AttribCmd {
    fn default() -> Self {
        Self::Help
    }
}


impl Cmd for AttribCmd {

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
        println!("{}", format!("Running attrib cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("
            ATTRIB: The attribute command allows for the look-\n
                    up or tagging of facts/items/records by u-\n
                    ser defined or automatically defined...\n
        ").color(Color::BrightRed);
        println!("> {}", help)
    }

}

impl FromArgMatches for AttribCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for AttribCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "attrib" {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Attrib {
    name: String,
}

impl Attrib {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

