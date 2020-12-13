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
                Self::new_cmd(),
                clap::App::new("list")
                    .about("List all of the facts globaally/in record/item")
                    .long_flag("ls")
                    .short_flag('l'),
                clap::App::new("search")
                    .about("Search for an fact from all items & records")
                    .long_flag("search")
                    .short_flag('s'),
                clap::App::new("info")
                    .about("Get info about a specific fact")
                    .long_flag("info")
                    .short_flag('i'),
                clap::App::new("link")
                    .about("Link two facts together, or with a record/fact")
                    .long_flag("link")
                    .short_flag('k')
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

impl FactCmd {

    pub fn new_cmd() -> clap::App<'static> {
        clap::App::new("new")
            .about("Create a new fact as a key-value pair")
            .long_flag("new")
            .short_flag('n')
            .aliases(&["add, create"])
            .args(&[
                clap::Arg::new("record")
                    .about("Specifies the record to add this new item to; inbox if none")
                    .aliases(&["r", "rec"])
                    .long("record")
                    .short('r')
                    .required(false)
                    .multiple(true)
                    .takes_value(true),
                clap::Arg::new("item")
                    .about("Specifies the item to add this new fact to")
                    .aliases(&["i", "itm"])
                    .long("item")
                    .short('i')
                    .required(false)
                    .multiple(true)
                    .takes_value(true),
                clap::Arg::new("attrib")
                    .about("Add any tags desired to the new item")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .multiple(true),
                clap::Arg::new("NAME")
                    .about("The name of the record to be added")
            ])
    }

    fn search_cmd() -> clap::App<'static> {
        clap::App::new("search")
            .about("Search for a fact")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
                    .multiple(true)
                    .required(false),
                clap::Arg::new("item")
                    .about("Filter by items")
                    .multiple(true)
                    .long("item")
                    .short('i')
                    .required(false),
                clap::Arg::new("record")
                    .about("Filter by record(s)")
                    .multiple(true)
                    .long("record")
                    .short('s')
                    .required(false)
            ])
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
