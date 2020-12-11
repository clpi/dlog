pub mod fact;
pub mod item;
pub mod record;

use std::collections::HashMap;
use self::{
    item::Item,
    fact::Fact,
    record::Record
};
use clap::{Arg, App, ArgMatches, SubCommand, crate_name, crate_authors, crate_version, crate_description};

#[derive(Debug)]
pub struct Dlog {
    pub logfile: String,
    pub opts: String,
}

#[derive(Debug)]
pub enum Command {
    Record,
    Item,
    Fact,
}

impl Dlog {

    pub fn run() {
        let matches = clap::app_from_crate!()
            .subcommands(vec![
                Item::cmd(),
                Record::cmd(),
                Fact::cmd(),
            ])
            .args(&vec![
                Self::help(),
                Self::version(),
            ])
            .get_matches();
        match matches.subcommand() {
            ("record", Some(sub)) => { Record::from(sub); },
            ("item", Some(sub)) => { Item::from(sub); },
            ("fact", Some(sub)) => { Fact::from(sub); },
            (&_, _) => { println!("No matches") }
        }
    }

    pub fn help() -> Arg<'static, 'static> {
        clap::Arg::with_name("help")
            .short("h")
            .long("help")
            .help("Print help info")
            .takes_value(false)
    }

    pub fn version() -> Arg<'static, 'static> {
        clap::Arg::with_name("version")
            .short("v")
            .long("version")
            .help("Print version info")
            .takes_value(false)
    }

    pub fn config_file() -> Arg<'static, 'static> {
        clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Manually set config file location and load")
            .takes_value(true)
    }

    pub fn matches() -> () {
    }

    pub fn print_help() {
        println!("dlog help")
    }
}
