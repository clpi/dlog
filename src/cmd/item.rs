use std::path::PathBuf;

use crate::util::get_input;
use colored::{Color, Colorize};
use super::{
    RecordCmd, Cmd,
    record::Record,
};
use clap::{ArgMatches, FromArgMatches};

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
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
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
