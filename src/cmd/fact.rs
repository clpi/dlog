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
            .about("facts")
            .subcommands(vec![
                Self::new_cmd(),
                Self::search_cmd(),
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
                    .index(1),
                clap::Arg::new("VALUE") //TODO if no index 3, prompt from stdin
                    .about("Value of the fact given by NAME")
                    .required(false)
                    .index(2),
                // clap::Arg::new("help")
                //     .about("Prints help for the fact command")
                //     .required(false)
                //     .exclusive(true)
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
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        println!("{}", format!("subc: {:#?}\n matches: {:#?}",
            matches.subcommand(),
            matches
        ).color(Color::BrightCyan));
        match matches.subcommand() {
            Some(("new", sub)) => {
                let _fact = Fact::from_arg_matches(sub);
            },
            Some(("search", sub)) => {
                if let Some(r_filts) = sub.values_of("filterrecord") {
                    println!("{}", "Filter fact in records:");
                    let r_filts = r_filts
                        .inspect(|r| {
                            println!("{}", format!("R: {}", r)
                                .color(Color::BrightCyan));
                        })
                        .collect::<Vec<&str>>();
                }
                if let Some(i_filts) = sub.values_of("filteritem") {
                    println!("{}", "Filter fact in items:");
                    let i_filts = i_filts
                        .inspect(|r| {
                            println!("{}", format!("R: {}", r)
                                .color(Color::BrightCyan));
                        })
                        .collect::<Vec<&str>>();
                }
            },
            Some(("list", sub)) => {
                println!("List facts comand");
            }
            Some(("info", sub)) => {
                println!("Info facts comand");
            }
            Some((&_, &_)) => {},
            None => {}
        }
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
                clap::Arg::new("NAME")
                    .about("Name of the fact to get or make")
                    .required(false)
                    .index(1),
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
                clap::Arg::new("filteritem")
                    .about("Filter by items")
                    .multiple(true)
                    .long("item")
                    .short('i')
                    .required(false),
                clap::Arg::new("filterrecord")
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
        let name = dialoguer::Input::new()
            .with_prompt("Fact name: ")
            .allow_empty(false)
            .validate_with(|input: &String| -> Result<(), &str> {
                let invalid = vec!["new", "search", "list", "info"];
                let inv_sym = vec!['@', '/', '&', '^', '$', '#'];
                for ch in inv_sym {
                    if input.contains(ch) {
                        return Err("Invalid character");
                    }
                }
                if invalid.contains(&input.as_str())
                    || input.len() > 40
                    || input.contains("\\") {
                    Err("Not a valid input")
                } else { Ok(()) }
            })
            .interact()
            .expect("Could not read user input");
        println!("{}", format!("Got new item: {}", &name)
            .color(Color::BrightCyan));
        Fact { name }
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match matches.value_of("NAME") {
            Some(name) => {
                println!("Got new fact: {}", &name);
                return Self { name: name.into() }
            },
            None => {
                println!("Received no fact name, provide: ");
                Self::default()
            }
        }
    }
}
