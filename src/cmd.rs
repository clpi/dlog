pub mod fact;
pub mod item;
pub mod record;
pub mod attribute;
pub mod link;
pub mod user;
pub mod stats;

use std::collections::HashMap;
use super::config::Config;
use self::{
    item::ItemCmd,
    fact::FactCmd,
    record::RecordCmd,
    link::LinkCmd,
    attribute::AttribCmd,
    user::UserCmd,
    stats::StatsCmd,
};
use colored::{Color, Colorize};
use clap::{Arg, ArgMatches, Clap, FromArgMatches};

pub enum App {
    Record(RecordCmd),
    Item(ItemCmd),
    Help,
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

impl App {

    pub fn run() {
        let _term = TermSettings::get();
        let _conf = Config::load();
        let matches = clap::app_from_crate!()
            .subcommands(vec![
                ItemCmd::cmd(),
                RecordCmd::cmd(),
                FactCmd::cmd(),
                AttribCmd::cmd(),
                LinkCmd::cmd(),
                StatsCmd::cmd(),
                UserCmd::cmd(),
            ])
            .args(&vec![
                Self::help(),
                Self::version(),
                Self::output(),
                Self::config_file(),
            ])
            .get_matches();
        println!("subc: {:#?}\n matches: {:#?}",
            matches.subcommand(),
            matches
        );
        // TODO handle this match through self-matching not here
        match matches.subcommand() {
            Some(("record", sub)) => RecordCmd::from_arg_matches(sub).run(),
            Some(("item", sub)) => ItemCmd::from_arg_matches(sub).run(),
            Some(("fact", sub)) => FactCmd::from_arg_matches(sub).run(),
            Some(("link", sub)) => LinkCmd::from_arg_matches(sub).run(),
            Some(("attrib", sub)) => AttribCmd::from_arg_matches(sub).run(),
            Some(("user", sub)) => UserCmd::from_arg_matches(sub).run(),
            Some(("stats", sub)) => StatsCmd::from_arg_matches(sub).run(),
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
        clap::Arg::new("output")
            .short('o')
            .long("output")
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

pub trait Cmd: FromArgMatches + Default {
    fn run(&self);
    fn cmd() -> clap::App<'static>;
    fn print_help();
}


#[derive(Clap, Debug)]
pub enum Command {
    Item,
    Record,
    Fact,
    Link,
    Attrib,
    Free,
}

impl std::str::FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "item" => Command::Item,
            "record" => Command::Record,
            "fact" => Command::Fact,
            "link" => Command::Link,
            "attrib" => Command::Attrib,
            _ => Command::Free,
        };
        Ok(res)
    }
}

