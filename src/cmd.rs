pub mod fact;
pub mod action;
pub mod item;
pub mod record;
pub mod attribute;
pub mod link;
pub mod user;
pub mod stats;
pub mod args;

use super::config::Config;
use self::{
    item::ItemCmd,
    fact::FactCmd,
    record::RecordCmd,
    link::LinkCmd,
    attribute::AttribCmd,
    user::UserCmd,
    stats::StatsCmd,
    action::ActionCmd,
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
    color: clap::AppSettings,
}

impl TermSettings {
    pub fn new() -> Self {
        let color = match std::env::var_os("NO_COLOR") {
            Some(_) => clap::AppSettings::ColoredHelp,
            None => clap::AppSettings::ColorNever,
        };
        let atty = atty::is(atty::Stream::Stdout);
        Self { atty, color }
    }
}

impl App {

    pub fn run() {
        let term = TermSettings::new();
        let _conf = Config::load();
        let matches = clap::app_from_crate!()
            .setting(term.color)
            .setting(clap::AppSettings::DeriveDisplayOrder)
            .subcommands(vec![
                ItemCmd::cmd(),
                RecordCmd::cmd(),
                FactCmd::cmd(),
                AttribCmd::cmd(),
                LinkCmd::cmd(),
                StatsCmd::cmd(),
                UserCmd::cmd(),
                ActionCmd::cmd(),
                clap::App::new("init")
                    .about("Initialize a fact database in the current folder"),
                clap::App::new("export")
                    .about("Export all of your data to a .zip file or HTML"),
                clap::App::new("inbox")
                    .about("Show operations related to unorganized facts and items"),
            ])
            .args(&vec![
                FactCmd::key_arg(1),
                FactCmd::val_arg(2),
                Self::help(),
                Self::version(),
                Self::output(),
                Self::config_file(),
            ])
            .get_matches();
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
            None => {
                println!("subc: {:#?}\n matches: {:#?}",
                    matches.subcommand(),
                    matches
                );
            }
        }
    }

    pub fn help() -> Arg<'static> {
        clap::Arg::new("help")
            .short('h')
            .long("help")
            .about("Print help info")
            .takes_value(false)
            .exclusive(true)
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

    pub fn set_config() -> Arg<'static> {
        clap::Arg::new("set")
            .short('s')
            .long("set")
            .about("Set a config key-value pair manually")
            .exclusive(true)
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

