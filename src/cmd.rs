pub mod fact;
pub mod item;
pub mod record;

use std::collections::HashMap;
use self::{
    item::Item,
    fact::Fact,
    record::Record
};
use clap::{Clap, Arg };

#[derive(Debug)]
pub struct Dlog {
    pub logfile: String,
    pub opts: String,
}

#[derive(Clap)]
pub struct Dl {
}

#[derive(Debug, Clap)]
pub enum Command {
    Record,
    Item,
    Fact,
}

pub struct TermSettings {
    atty: bool,
}

impl TermSettings {
    pub fn get() -> Self {
        Self {
            atty: atty::is(atty::Stream::Stdout),
        }
    }
}

impl Dlog {

    pub fn run() {
        let _term = TermSettings::get();
        let matches = clap::app_from_crate!()
            .subcommands(vec![
                Item::cmd(),
                Record::cmd(),
                Fact::cmd(),
            ])
            .args(&vec![
                Self::help(),
                Self::version(),
                Self::output(),
                Self::config_file(),
            ])
            .get_matches();
        match matches.subcommand() {
            Some(("record", sub)) => { Record::from(sub); },
            Some(("item", sub)) => { Item::from(sub); },
            Some(("fact", sub)) => { Fact::from(sub); },
            Some((&_, &_)) => {},
            None => { println!("No matches") }
        }
    }

    pub fn help() -> Arg<'static> {
        clap::Arg::new("help")
            .short('h')
            .long("help")
            .about("Print help info")
            .takes_value(false)
    }

    pub fn version() -> Arg<'static> {
        clap::Arg::new("version")
            .short('v')
            .long("version")
            .about("Print version info")
            .takes_value(false)
    }

    pub fn output() -> Arg<'static> {
        clap::Arg::new("version")
            .short('v')
            .long("version")
            .about("Print version info")
            .takes_value(true)
    }

    pub fn match_output(val: String) {
        match val.as_str() {
            "json" => { println!("JSON output") },
            "yaml" => { println!("YAML output") }
            _ => { println!("Invalid output type") }
        }
    }

    pub fn config_file() -> Arg<'static> {
        clap::Arg::new("config")
            .short('c')
            .long("config")
            .about("Manually set config file location and load")
            .takes_value(true)
    }

    pub fn matches() -> () {
    }

    pub fn print_help() {
        println!("dlog help")
    }
}
