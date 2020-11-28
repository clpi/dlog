use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct Field {
    key: String,
}

impl SubCommand for Field {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightBlue }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        println!("{}", format!("F: {:#?}", key).color(Field::color()));
        println!("{:#?}", args);
        Ok(Self { key} )
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
