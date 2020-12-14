use std::{path::PathBuf, fs};
use chrono::{Utc, DateTime};
use crate::{
    util,
    cmd::{ Cmd,
        item::Item,
        fact::Fact,
    }
};
use clap::{ArgMatches, FromArgMatches, Subcommand};
use colored::{Color, Colorize, Style, Styles};

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

    pub fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            Self { name }
        } else {
            Self::default()
        }
    }

    pub fn get_or_create(&self) -> std::io::Result<PathBuf> {
        let rec_dir = util::get_or_create_data_dir()?
            .join(&self.name);
        if rec_dir.exists() && rec_dir.is_dir() {
            let rec = rec_dir.join(&format!("{}.csv", &self.name));
            if rec.exists() && rec.is_file() {
                return Ok(rec)
            } else {
                fs::File::create(&rec)?;
                return Ok(rec)
            }
        } else {
            fs::create_dir(&rec_dir)?;
            let rec = rec_dir.join(&format!("{}.csv", &self.name));
            fs::File::create(&rec)?;
            Ok(rec)
        }
    }

    pub fn add_item(&self, item: &Item) -> std::io::Result<PathBuf> {
        let rec = self.get_or_create()?;
        let item = rec.parent().expect("Could not find parent")
            .join(format!("{}{}", &item.name, ".csv"));
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_path(&item)?;
        Ok(item)
    }

    pub fn read(&self, item: Option<String>) -> std::io::Result<Vec<Fact>> {
        let rec = self.get_or_create()?;
        let csv = if let Some(item) = item {
            PathBuf::from(rec).join(&format!("{}.csv", &item))
        } else {
            rec
        };
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .double_quote(false)
            .escape(Some(b'\\'))

            .from_path(&csv)?;
        let _headers = rdr.headers()?.clone();
        while let Some(rec) = rdr.records().next() {
            match rec {
                Ok(rec) => {
                    println!("{:#?}", rec);
                    let fact = Fact {
                        name: rec[0].to_string(),
                        val: rec[1].to_string(),
                        time: DateTime::parse_from_rfc2822(&rec[2])
                            .expect("Could not parse datetime").into(),
                    };
                    while let Some(attr) = rec.iter().skip(3).next() {

                    }
                },
                Err(e) => return Err(From::from(e)),
            };
        }
        Ok(vec![Fact::default()])
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
