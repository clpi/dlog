use std::path::PathBuf;

use crate::util::get_input;
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
                clap::App::new("info")
                    .about("Get info about a specific item")
                    .long_flag("info")
                    .short_flag('i'),
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
                clap::Arg::new("uncategorized")
                    .aliases(&["misc", "uncat", "etc"])
                    .short('u')
                    .long("uncategorized")
                    .about("Whether to show items part of the inbox record")
                    .takes_value(false)
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

#[derive(Debug)]
pub struct Item {
    pub name: String,
    record: Record,
}

impl Default for Item {
    fn default() -> Self {
        let name = get_input().expect("Could not read item name");
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
        Self::default()
    }
}
