pub mod record;
pub mod item;
pub mod field;
pub mod help;
pub mod list;
pub mod config;

use colored::Colorize;
use pico_args::Arguments;
use self::{
    record::RecordCmd,
    item::ItemCmd,
    field::FieldCmd,
    help::HelpCmd,
    list::ListCmd,
    config::ConfigCmd
};

#[derive(Debug, Default)]
pub struct LogCmd {
    help: bool,
    version: bool,
    record: Option<String>,
    item: Option<String>,
    field: Option<String>,
    value: Option<String>,
    units: Option<String>,
    free: Vec<String>,
}

impl LogCmd {

    pub fn parse() -> Result<Self, pico_args::Error> {
        let mut args = pico_args::Arguments::from_env();
        println!("{:#?}", args.clone());
        let cmd = match args.subcommand()? {
            Some(cmd) => {
                match cmd.as_str() {
                    "help" => HelpCmd::parse(&mut args)?,
                    "list" => ListCmd::parse(&mut args)?,
                    "record" => RecordCmd::parse(&mut args)?,
                    "item" => ItemCmd::parse(&mut args)?,
                    "field" => FieldCmd::parse(&mut args)?,
                    "config" => ConfigCmd::parse(&mut args)?,
                    "new" => Self::parse_new(&mut args)?,
                    "version" => Self::print_version(),
                    _ => RecordCmd::new(cmd, &mut args)?,
                };
                Self::default()
            },
            None => {
                let log = Self {
                    help: args.contains(["-h", "--help"]),
                    version: args.contains(["-v", "--version"]),
                    record: args.opt_value_from_str(["-r", "--record"])?,
                    item: args.opt_value_from_str(["-i", "--item"])?,
                    free: args.free()?,
                    ..Default::default()
                };
                log
            }
        };
        println!("{:#?}", cmd);
        Ok(cmd)
    }

    pub fn items(args: &Arguments) -> Result<(), pico_args::Error> {
        Ok(())
    }

    pub fn records(args: &Arguments) -> Result<(), pico_args::Error> {
        Ok(())
    }

    pub fn fields(args: &Arguments) -> Result<(), pico_args::Error> {
        Ok(())
    }

    pub fn print_version() {
        println!("{}", format!("dlog version {}", "0.1.0")
            .bold().bright_purple());
    }

    pub fn parse_new(args: &mut Arguments) -> Result<(), pico_args::Error> {
        match (args.subcommand()?, args.subcommand()?) {
            (Some(kind), Some(name)) => {
                match kind.as_str() {
                    "item" => {ItemCmd::new(name, args)?;},
                    "record" => {RecordCmd::new(name, args)?;},
                    "field" => {FieldCmd::new(name, args)?;},
                    _ => {Self::parse()?;},
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

pub enum DataKind {
    Record(String),
    Item(String),
    Field(String)
}

pub enum SubCmd {
    Data(DataKind),
    List(DataKind),
    Help,
    Version,
}

impl SubCmd {

    pub fn get(args: &Arguments) -> Self {
        Self::Help
    }

    pub fn help(args: &Arguments) -> bool {
        true
    }
}
