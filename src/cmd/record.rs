use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct Record {
    key: String,
}

impl Record {

    pub fn self_sans_key() -> Self {
        Self { key: "Uncategorized".into() }
    }

    pub fn init(key: String) -> Self {
        Self { key }
    }
}

impl SubCommand for Record {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightCyan }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        println!("{}", format!("R: {:#?}", key).color(Record::color()));
        println!("{:#?}", args);
        Ok(Self { key })
    }
}

impl Default for Record {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for Record {
    fn to_string(&self) -> String {
        "record".to_string()
    }
}
