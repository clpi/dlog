use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct Item {
    key: String
}

impl Item {

    pub fn init(key: String) -> Self {
        Self { key }
    }
}

impl SubCommand for Item {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightRed }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if args.clone().free()?.is_empty() {
            return Self::init(key).prompt_value();
        }
        println!("{}", format!("I: {:#?}", key).color(Item::color()));
        println!("{:#?}", args);
        Ok(Self { key })
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
