pub mod fact;
pub mod item;
pub mod record;
pub mod list;
pub mod config;
pub mod link;

use colored::{Colorize, Color, Style, Styles};
use std::env::{Args, args};
use pico_args::Arguments;
use crate::cmd::{
        record::Record,
        item::Item,
        fact::Fact,
        list::List,
        config::Config,
        link::Link,
    };
use comfy_table::{
    Table,
    presets::UTF8_BORDERS_ONLY,
    modifiers::UTF8_ROUND_CORNERS,
};

#[derive(Debug)]
pub struct Log {}

impl Log {

    pub fn run() -> Result<(), pico_args::Error> {
        let mut args = pico_args::Arguments::from_env();
        Self::parse(&mut args)
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        // println!("{:#?}", args.clone()); NOTE use to diagnose args
        if args.contains(["-h", "--help"]) {
            Self::print_help("");
        } else if args.contains(["-v", "--version"]) {
            Self::print_version();
            args.clone().finish()?

        } else {
            while let Some(cmd) = args.subcommand()? {
                match cmd.as_str() {
                    "record" | "r" => {Record::parse(args)?;},
                    "item" | "i" => {Item::parse(args)?;},
                    "fact" | "f" => {Fact::parse(args)?;},
                    "config" | "c"=> {Config::parse(args)?;},
                    "link" | "l" => { Link::parse(args)?; break },
                    "list" | "ls" => { List::parse(args)?; break},
                    "new" | "n" => { Self::parse_new(args)?; break },
                    "version" | "v" => { Self::print_version(); break }, // be flag
                    "help" | "h"   => { Self::print_help(cmd.as_str()); break },
                    _ => {Fact::with_args(Some(cmd.to_string()), args)?; break},
                };
            };
        }
        Ok(())
    }

    pub fn parse_flags(mut _args: Arguments) -> Result<(), pico_args::Error> {
        Ok(())
    }

    pub fn print_version() {
        println!("{}", format!("dlog version {}", "0.1.0")
            .bold().bright_purple());
    }

    pub fn print_help(cmd: &str) {
        match cmd {
            "" => { println!("dlog version 0.1.0 help") },
            "record" | "r" => { Record::help() },
            "item" | "i" => { Item::help() },
            "Fact" | "f" => { Fact::help() },
            "config" | "c"=> { Config::help() },
            "link" | "l" => { Link::help() },
            "list" | "ls" => { List::help() },
            "new" | "n" => { println!(" new help ") },
            _ => { println!(" invalid help ") },
        };
        println!("{}", format!("dlog version {}", "0.1.0")
            .bold().bright_purple());
    }

    pub fn parse_new(args: &mut Arguments) -> Result<(), pico_args::Error> {
        match (args.subcommand()?, args.subcommand()?) {
            (Some(kind), Some(name)) => {
                match kind.as_str() {
                    "item" => {Item::with_args(Some(name), args)?;},
                    "record" => {Record::with_args(Some(name), args)?;},
                    "Fact" => {Fact::with_args(Some(name), args)?;},
                    _ => (),
                };
            },
            _ => {
                let mut item = String::new();
                std::io::stdin().read_line(&mut item).unwrap();
                println!("{}", format!("Created item {}", item)
                    .bright_red());
            }
        };
        Ok(())
    }

    fn _parse_free(args: Vec<String>) -> () {

    }

}

pub trait SubCommand: ToString + Default {

    fn cmd_string() -> Vec<&'static str>;

    fn color() -> Color;

    fn printclr(input: String, bold: bool, italic: bool, blink: bool) {
        let mut input = input.color(Self::color());
        if bold { input = input.bold() }
        if italic { input = input.italic() }
        if blink { input = input.blink() }
        println!("{}", input)
    }

    fn new(key: String, val: Option<String>) -> Self;

    fn get(_args: &Arguments) -> () {}

    fn help() -> () {}

    fn insert(&self) -> Result<(), pico_args::Error>;

    fn prompt_key() -> Result<String, pico_args::Error> {
        let mut key = String::new();
        println!("{}", format!("What is the {:?} name?: ", Self::cmd_string()[0])
            .color(Self::color()));
        std::io::stdin().read_line(&mut key).unwrap();
        println!("{}", format!("Got new {:?}: {:?}: ", Self::cmd_string()[0],key)
            .color(Self::color()));
        Ok(key) //TODO process value to item, Fact, etc
    }

    fn prompt_value(self) -> Result<String, pico_args::Error> {
        let mut buf = String::new();
        println!("{}", format!("What is the {:?} val?: ", Self::cmd_string()[0])
            .color(Self::color()));
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("{}", format!("Got {:?}: {:?}: ", Self::cmd_string()[0], buf)
            .color(Self::color()));
        Ok(buf)
    }

    fn parse(args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if let Some(key) = args.subcommand()? {
            match key.as_str() {
                "new" => {Self::with_args(args.subcommand()?, args)?;},
                "list" =>{ List::kind(Self::cmd_string()[0]); },
                "help" => {Log::print_help(Self::cmd_string()[0]);},
                "link" => { Link::with_args(Some(key.clone()), args)?; },
                _ =>  {},
            };
            Ok(Self::with_args(Some(key.clone()), args)?)
        } else {
            Ok(Self::new("".into(), None))
        }
    }

    fn with_args(key: Option<String>, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        match (key, args.subcommand()?) {
            (Some(mut key), Some(val)) => {
                if Self::cmd_string().contains(&val.as_str()) {
                    key  = val;
                    return Self::with_args(Some(key.clone()), args);
                }
                if Self::cmd_string().contains(&key.as_str()) {
                    return Self::with_args(Some(val.clone()), args);
                }
                println!("{}", format!("{}: {:?} = {:?}",
                        Self::cmd_string()[0], key, val)
                    .color(Self::color()));
                let new = Self::new(key, Some(val));
                new.insert()?;
                Ok(new)
            }
            (Some(mut key), None) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    key = Self::prompt_key()?;
                }
                let val = Self::new(key.clone(), None).prompt_value()?;
                println!("{}", format!("{}: {:?} = {}",
                        Self::cmd_string()[0], key, val.clone())
                    .color(Self::color()));
                let new = Self::new(key, Some(val));
                new.insert()?;
                Ok(new)
            }
            _ => {
                Ok(Self::default())
            }
        }
    }

    fn show_in_table<T: ToString>(rows: Vec<Vec<T>>, cols: Vec<String>) {
        let mut table = Table::new();
        table.load_preset(UTF8_BORDERS_ONLY);
        table.apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(cols);
        for row in rows {
            let row: Vec<String> = row.iter().map(|c| c.to_string()).collect();
            table.add_row(row);
        }

    }

}
