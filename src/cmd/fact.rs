use crate::{
    csv as Csv, util::prompt_input,
    models::{
        fact::Fact,
        units::Units,
        item::Item,
        attrib::Attrib,
    },
    cmd::Cmd
};
use std::{fmt, path::PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use clap::{ArgMatches, FromArgMatches};
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
            .about("The fact subcommand")
            .long_about("A fact is a ...")
            .subcommands(vec![
                Self::new_cmd(),
                Self::search_cmd(),
                Self::list_cmd(),
                clap::App::new("get")
                    .about("Get info about a specific fact")
                    .long_flag("get")
                    .short_flag('g'),
                clap::App::new("link")
                    .about("Link two facts together, or with a record/fact")
                    .long_flag("link")
                    .short_flag('k')
            ])
            .args(&vec![
                Self::key_arg(1),
                Self::val_arg(2),
                Self::val_unit(3),
                Self::set_units(),
                clap::Arg::new("attribs")
                    .about("Add any attribs desired to the new fact")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .validator(|a| crate::util::validate_input(a.into()))
                    .multiple(true),

                clap::Arg::new("record")
                    .about("Specify the record to add this fact to")
                    .long("record")
                    .overrides_with("record")
                    .short('r')
                    .required(false)
                    .settings(&[
                        clap::ArgSettings::UseValueDelimiter
                    ])
                    .multiple(true),
                clap::Arg::new("item")
                    .about("Specify the item to add this fact to")
                    .long("item")
                    .overrides_with("item")
                    .short('i')
                    .required(false)
                    .settings(&[
                        clap::ArgSettings::UseValueDelimiter
                    ])
                    .multiple(true),
                clap::Arg::new("link-attribute")
                    .long("Whether to persist the attribute-fact link")
                    .long_about("Link an attribute to this fact (not just this fact entry)")
                    .long("link-attrib")
                    .aliases(&["save-attrib",  "attrib-link"])
                    .short('A')
                    .overrides_with("attribs") //TODO test this
                    .multiple(true)
                    .required(false),
                clap::Arg::new("link-item")
                    .about("Whether to persist the item-fact link specified")
                    .long_about("Link an item to this fact (not just this fact entry)")
                    .long("link-item")
                    .aliases(&["save-item",  "item-link"])
                    .short('I')
                    .overrides_with("item") //TODO test this
                    .multiple(true)
                    .required(false),
                clap::Arg::new("link-record")
                    .about("Whether to persist the record-fact link specified")
                    .long_about("Link a record to this fact (not just this fact entry)")
                    .long("link-record")
                    .aliases(&["save-record", "save-rec", "record-link"])
                    .short('R')
                    .overrides_with("record") //TODO test this
                    .multiple(true)
                    .required(false),
            ]) // TODO add fact-fact link possibility
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
        println!("{}", format!("subc: {:#?} \n matches: {:#?}",
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
                    .requires("VALUE")
                    .takes_value(true),
                clap::Arg::new("newattribs")
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
                    .required(false),
                clap::Arg::new("case-insensitive")
                    .about("Search for fact case insensitive")
                    .required(false),
                clap::Arg::new("max-results")
                    .long("max-results")
                    .takes_value(true)
                    .value_name("rescount")
                    .hidden_short_help(true)
                    .long_about("Limit the number of search results to 'count' and quit immediately."),
            ])
    }

    fn list_cmd() -> clap::App<'static> {
        clap::App::new("list")
            .about("List all of the facts globaally/in record/item")
            .long_about("Specify arguments to list different facts")
            .long_flag("ls")
            .short_flag('l')
            .args(&[
                clap::Arg::new("record")
                    .about("Fact in record")
                    .short('r'),
                clap::Arg::new("item")
                    .about("Fact in item")
                    .short('i'),
                clap::Arg::new("attribute")
                    .about("Fact with attribute")
                    .short('a'),
            ])
    }

    pub fn key_arg(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("NAME")
            .about("Name of the fact to get or make")
            .required(false)
            .validator(|a| crate::util::validate_input(a.into()))
            .index(idx)
    }

    pub fn val_arg(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("VALUE") //TODO if no index 3, prompt from stdin
            .requires("NAME")
            .about("Value of the fact given by NAME")
            .required(false)
            .validator(|a| crate::util::validate_input(a.into()))
            .index(idx)
    }

    pub fn val_unit(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("UNIT") //TODO if no index 3, prompt from stdin
            .about("First unit value")
            .long_about("Units for the value provided for the input fact. If not provided, defaults to the last units provided or the units specified as the permanent units for this fact.")
            .multiple(true)
            .use_delimiter(true)
            .value_delimiter(" ")
            .require_delimiter(true)
            .requires_all(&["VALUE", "NAME"])
            .required(false)
            .validator(|a| crate::util::validate_input(a.into()))
            .index(idx)
    }

    pub fn set_units() -> clap::Arg<'static> {
        clap::Arg::new("set-units") //TODO if no index 3, prompt from stdin
            .hidden_short_help(true)
            .long_about("Save the defined units as the permanent units for this fact")
            .aliases(&["set-unit", "save-unit"])
            .short('U')
            .long("set-units")
            .requires_all(&["VALUE", "NAME", "UNIT"])
            .required(false)
    }
}


impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        if let Some(name) = matches.value_of("NAME") {
            println!("Got new fact: {}", &name);
            if let Some(value) = matches.value_of("VALUE") {
                println!("Got new fact: {} = {}", &name, &value);
                let units: Units = if let Some(units)
                    = matches.values_of("UNIT")
                {
                    if matches.occurrences_of("UNIT") == 1 {
                        Units::Other(units.take(0).collect())
                    } else {
                        let units = units.into_iter().collect();
                        Units::Other(units)
                    }
                } else { Units::None };
                println!("Got new fact: {} = {} ({})", &name, &value, &units);
                let attribs: Vec<Attrib> =
                    if let Some(attribs) = matches.values_of("attribs") {
                    attribs.map(|a| Attrib::new(a))
                        .collect()
                    } else { Vec::new() };
                return Self::new(name.into(), value.into(), units, attribs)
            } else {
                return Self { name: name.into(), ..Self::default()  }
            }
        } else {
            println!("Received no fact name, provide: ");
            Self::default()
        }
    }
}





