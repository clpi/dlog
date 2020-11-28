pub mod record;
pub mod item;
pub mod field;
pub mod list;
pub mod config;
pub mod link;

use colored::{Colorize, Color};
use pico_args::Arguments;
use crate::{
    cmd::{
        record::Record,
        item::Item,
        field::Field,
        list::List,
        config::Config,
        link::Link,
    },
    types::DataType,
};

#[derive(Debug)]
pub struct Log {}

impl Log {

    pub fn run() -> Result<(), pico_args::Error> {
        let mut args = pico_args::Arguments::from_env();
        Self::parse(&mut args)
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{:#?}", args.clone());
        if args.contains(["-h", "--help"]) {
            Self::print_help("");
        } else if args.contains(["-v", "--version"]) {
            Self::print_version();
            args.clone().finish()?
        } else {
            while let Some(cmd) = &mut args.subcommand()? {
                match cmd.as_str() {
                    "record" | "r" => {Record::parse(args)?;},
                    "item" | "i" => {Item::parse(args)?;},
                    "field" | "f" => {Field::parse(args)?;},
                    "config" | "c"=> {Config::parse(args)?;},
                    "link" | "l" => { Link::parse(args)?; },
                    "list" | "ls" => { List::parse(args)?; },
                    "new" | "n" => { Self::parse_new(args)?; },
                    "version" | "v" => { Self::print_version() }, // be flag
                    "help" | "h"   => { Self::print_help(cmd)},
                    _ => {Item::new(cmd.clone()).prompt_value()?;},
                };
            };
        }
        println!("{:#?}", args.clone());
        Ok(())
    }

    pub fn parse_flags(mut args: Arguments) -> Result<(), pico_args::Error> {
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
            "field" | "f" => { Field::help() },
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
                    "item" => {Item::with_args(name, args)?;},
                    "record" => {Record::with_args(name, args)?;},
                    "field" => {Field::with_args(name, args)?;},
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

    fn parse_free(args: Vec<String>) -> () {

    }

}

pub trait SubCommand: ToString + Default {

    fn color() -> Color;

    fn new(key: String) -> Self;

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error>;

    fn get(args: &Arguments) -> () {}

    fn help() -> () {}

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error>;

    fn prompt_key() -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn prompt_value(self) -> Result<Self, pico_args::Error> {
        let mut buf = String::new();
        println!("{}", format!("What is {} val?: ", self.to_string())
            .color(Self::color()));
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("{}", format!("Got {}: ", buf)
            .color(Self::color()));
        Ok(Self::new(buf))
    }

    fn parse(args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if let Some(key) = args.clone().free()?.get(0) {
            Self::with_args(key.clone(), args)
        } else {
            let key = Self::prompt_key()?;
            key.prompt_value()
        }
    }
}
