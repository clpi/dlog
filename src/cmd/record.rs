use std::{path::PathBuf, fs};
use crate::util;
use clap::{ArgMatches, FromArgMatches, Subcommand};
use colored::{Color, Colorize, Style, Styles};
use super::{Cmd, item::Item};

#[derive(Debug)]
pub enum RecordCmd {
    New(Option<Record>),
    List
}

impl Default for RecordCmd {
    fn default() -> Self {
        RecordCmd::New(None)
    }
}

impl Cmd for RecordCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("record")
            .about("items")
            .subcommands(vec![
                Self::new_cmd(),
                Self::search_cmd(),
                clap::App::new("list")
                    .about("List all records")
                    .long_flag("ls")
                    .short_flag('l'),
                clap::App::new("info")
                    .about("Get info about a specific record")
                    .long_flag("info")
                    .short_flag('i'),
                clap::App::new("link")
                    .about("Link two records together, or with a item/fact")
                    .long_flag("link")
                    .short_flag('k')
            ])
            .args(&vec![
                clap::Arg::new("help")
                    .about("Display help pertaining to records")
                    .short('h')
                    .long("help")
                    .takes_value(false),
            ])

    }

    fn run(&self) {
        println!("{}", format!("Running record cmd...")
            .color(Color::BrightGreen))
    }

    fn print_help() {
        let help = format!("
            RECORD: Define a grouping of items to keep track \n
                    of in different records, and assign diff-\n
                    erent actions and attributes based on... \n
        ").color(Color::BrightGreen);
        println!("> {}", help)
    }
}

impl FromArgMatches for RecordCmd {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}

// NOTE Don't think this is supposed to be implemented yet
//      until functionality is finished for clap 3.0 but
//      implementing anyways
impl Subcommand for RecordCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            if sub == "record" {
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

impl RecordCmd {
    fn new_cmd() -> clap::App<'static> {
        clap::App::new("new")
            .about("Create a new record")
            .long_flag("new")
            .short_flag('n')
            .aliases(&["create"])
            .args(&[
                clap::Arg::new("record")
                    .about("Specifies the record to add this new item to; inbox if none")
                    .aliases(&["r", "rec"])
                    .long("record")
                    .short('r')
                    .required(false)
                    .index(1)
                    .takes_value(true)
                    .multiple(true),
                clap::Arg::new("attrib")
                    .about("Add any tags desired to the new record")
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
            .about("Search for a record")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
                    .required(false),
            ])
    }
}

#[derive(Debug)]
pub struct Record {
    pub name: String,
}

impl Default for Record {
    fn default() -> Self {
        Self { name: "Uncategorized".into() }
    }
}

impl Record {

    fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            Self { name }
        } else {
            Self::default()
        }
    }

    fn create(&self) -> std::io::Result<PathBuf> {
        let data_dir = util::get_or_create_data_dir()?
            .join(&self.name);
        fs::create_dir(&data_dir)?;
        Ok(data_dir)
    }

    pub fn add_item(&self, item: &Item) -> std::io::Result<PathBuf> {
        let record_dir = util::get_or_create_data_dir()?
            .join(&self.name)
            .join(format!("{}{}", &item.name, ".csv"));
        fs::File::create(&record_dir)?;
        Ok(record_dir)
    }

}

impl From<String> for Record {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl FromArgMatches for Record {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}
