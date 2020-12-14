use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::util::prompt_input;
use colored::{Color, Colorize};
use super::{
    RecordCmd, Cmd,
    record::Record,
};
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug)]
pub enum ItemCmd {
    New(Option<Item>),
    Delete(Option<Item>),
    List,
    Help,
}

impl Default for ItemCmd {
    fn default() -> Self {
        ItemCmd::New(None)
    }
}

impl Cmd for ItemCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("item")
            .about("items")
            .subcommands(vec![
                Self::new_cmd(),
                Self::search_cmd(),
                clap::App::new("list")
                    .about("List all of the items globaally or in a record")
                    .long_flag("ls")
                    .short_flag('l'),
                clap::App::new("get")
                    .about("Get info about a specific item")
                    .long_flag("get")
                    .short_flag('g'),
                clap::App::new("link")
                    .about("Link two items together, or with a record/fact")
                    .long_flag("link")
                    .short_flag('k')
            ])
            .args(&vec![
                clap::Arg::new("help")
                    .about("Display help pertaining to items")
                    .short('h')
                    .long("help")
                    .takes_value(false)
                    .exclusive(true),
                clap::Arg::new("NAME")
                    .about("Name of the item to log")
                    .required(false)
                    .index(1),
                clap::Arg::new("FACT") //TODO if no index 3, prompt from stdin
                    .about("Optional fact to associate with new item")
                    .required(false)
                    .index(2),
                clap::Arg::new("FACTVAL") //TODO if no index 3, prompt from stdin
                    .about("Optional value of the fact to associate with new fact")
                    .required(false)
                    .index(3),
                clap::Arg::new("uncategorized")
                    .aliases(&["misc", "uncat", "etc"])
                    .short('u')
                    .long("uncategorized")
                    .about("Whether to show items part of the inbox record")
                    .takes_value(false),
                clap::Arg::new("attribs")
                    .about("Add any attribs desired to the new item")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .multiple(true),
                clap::Arg::new("record")
                    .about("Specify the record to add this fact to")
                    .long("record")
                    .short('r')
                    .required(false)
                    .multiple(true),
            ])
    }

    fn run(&self) {
        println!("{}", format!("Running item cmd...")
            .color(Color::BrightMagenta))

    }

    fn print_help() {
        let help = format!("
            ITEM: Items define groups of related facts into\n
                  logical clusterings which map to real world\n
                  blah blah blah to be added later\n
        ").color(Color::BrightMagenta);
        println!("> {}", help)
    }

}

impl ItemCmd {
    pub fn new_cmd() -> clap::App<'static> {
        clap::App::new("new")
            .about("Create a new item to associate with different facts")
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
                clap::Arg::new("attrib")
                    .about("Add any tags desired to the new item")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .multiple(true),
                clap::Arg::new("NAME")
                    .about("The name of the item to be added")
                    .required(false)
            ])
    }

    fn search_cmd() -> clap::App<'static> {
        clap::App::new("search")
            .about("Search for an item")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
                    .multiple(true)
                    .required(false),
                clap::Arg::new("record")
                    .about("Filter by record(s)")
                    .multiple(true)
                    .long("record")
                    .short('s')
                    .multiple(true)
                    .required(false)
            ])
    }
}

impl FromArgMatches for ItemCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for ItemCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            if sub == "item" {
                Some(Self::from_arg_matches(args))
            } else {
                None
            }
        } else { None }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Item")]
    pub name: String,
    pub record: Record,
}

impl Default for Item {
    fn default() -> Self {
        // TODO make this into a function called by all default functions
        let name = prompt_input("Item name: ")
            .expect("Could not prompt item name");
        println!("{}", format!("Got new item: {}", &name)
            .color(Color::BrightCyan));
        Item { name, record: Record::default() }
    }
}

impl Item {

    pub fn new(name: String, record: Option<String>) -> Self {
        if let Some(record) = record {
            Self { name, record: Record::from(record) }
        } else {
            Self { name, record: Record::default() }
        }
    }

    pub fn create(&self) -> std::io::Result<PathBuf> {
        let item = self.record.add_item(self)?;
        Ok(item)
    }

}

impl FromArgMatches for Item {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match (matches.value_of("NAME"), matches.value_of("record")) {
            (Some(item), Some(record)) => {
                Self {
                    name: item.into(),
                    record: Record::from(record.to_string())
                }
            },
            (Some(item), None)  => {
                Self {
                    name: item.into(),
                    record: Record::default()
                }
            },
            (_, _) => Self::default(),
        }
    }
}
