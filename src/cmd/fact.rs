use crate::util::get_input;
use clap::{ArgMatches, FromArgMatches};
use super::{
    Cmd,
    item::Item,
};
use colored::{Color, Colorize};

#[derive(Debug)]
pub enum FactCmd {
    New(Fact),
    Delete(Fact),
    LinkFact(Fact),
    LinkItem(Item),
    Help,
    List,
}

impl Default for FactCmd {
    fn default() -> Self {
        FactCmd::Help
    }
}

impl Cmd for FactCmd {

    fn run(&self) {
        println!("{}", format!("Running fact cmd...")
            .color(Color::BrightCyan))
    }

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
                clap::Arg::new("VALUE") //TODO if no index 3, prompt from stdin
                    .about("Value of the fact given by NAME")
                    .required(false)
                    .index(3),
                clap::Arg::new("help")
                    .about("Prints help for the fact command")
            ])
    }

    fn print_help() {
        let help = format!("
            FACT: A fact is at its most basic, a key-value pa-\n
                  ir which defines a single piece of info blah\n
                  blah write later                            \n
        ").color(Color::BrightCyan);
        println!("> {}", help)
    }
}

impl FromArgMatches for FactCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for FactCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "fact" {
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

#[derive(Debug)]
pub struct Fact {
    name: String,
}

impl Default for Fact {
    fn default() -> Self {
        let name = get_input().expect("Could not read item name");
        Fact { name }
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::default()
    }
}
