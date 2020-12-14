use crate::{
    util::prompt_input,
    cmd::{
        Cmd,
        item::Item,
        record::Record,
        attribute::Attrib,
    }
};
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
                clap::Arg::new("NAME")
                    .about("Name of the fact to get or make")
                    .required(false)
                    .validator(|a| crate::util::validate_input(a.into()))
                    .index(1),
                clap::Arg::new("VALUE") //TODO if no index 3, prompt from stdin
                    .about("Value of the fact given by NAME")
                    .required(false)
                    .validator(|a| crate::util::validate_input(a.into()))
                    .index(2),
                clap::Arg::new("attribs")
                    .about("Add any attribs desired to the new fact")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .use_delimiter(true)
                    .validator(|a| crate::util::validate_input(a.into()))
                    .settings(&[
                        clap::ArgSettings::UseValueDelimiter,
                        clap::ArgSettings::AllowHyphenValues,
                    ])
                    .multiple(true),

                clap::Arg::new("record")
                    .about("Specify the record to add this fact to")
                    .long("record")
                    .short('r')
                    .required(false)
                    .settings(&[
                        clap::ArgSettings::UseValueDelimiter
                    ])
                    .multiple(true),
                clap::Arg::new("item")
                    .about("Specify the item to add this fact to")
                    .long("item")
                    .short('i')
                    .required(false)
                    .settings(&[
                        clap::ArgSettings::UseValueDelimiter
                    ])
                    .multiple(true),
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
                    .required(false)
            ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Value")]
    pub val: String,
    #[serde(rename="Datetime")]
    pub time: DateTime<Utc>,
    pub attribs: Vec<Attrib>,
}

impl Fact {

    pub fn new(name: String, val: String, attribs: Option<Vec<Attrib>>) -> Self {
        let attribs = attribs.unwrap_or_default();
        Self { name, val, time: Utc::now(), attribs }
    }

    pub fn write(
        record: Option<Record>,
        item: Option<Item>
    ) -> std::io::Result<()>
    {
        match (record, item) {
            (Some(record), Some(item)) => {

            },
            (Some(record), None) => {
                let mut wtr = crate::csv::csv_writer(record.get_or_create()?);

            },
            (None, Some(item)) => {

            }
            (None, None) => {

            }
        };
        Ok(())
    }
}

impl Default for Fact {
    fn default() -> Self {
        let name = prompt_input("Fact name: ")
            .expect("Could not prompt fact name");
        let val = prompt_input("Fact value: ")
            .expect("Could not prompt fact value");
        println!("{}", format!("Got new item: {} = {}",
                &name, &val).color(Color::BrightCyan));
        Fact { name, val, time: Utc::now(), attribs: Vec::new() }
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match matches.value_of("NAME") {
            Some(name) => {
                println!("Got new fact: {}", &name);
                if let Some(value) = matches.value_of("VALUE") {
                    let attribs: Vec<Attrib> =
                        if let Some(attribs) = matches.values_of("attribs") {
                        attribs.map(|a| Attrib::new(a))
                            .collect()
                        } else { Vec::new() };
                    return Self::new(name.into(), value.into(), Some(attribs));
                } else {
                    return Self::new(
                        name.into(),
                        prompt_input(format!("What is {}'s value?: ", &name).as_str()).expect("Could not read fact value"),
                        None)
                }
            },
            None => {
                println!("Received no fact name, provide: ");
                Self::default()
            }
        }
    }
}
