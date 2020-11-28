use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct List {
    key: String,
}

impl List {

    pub fn init(key: String) -> Self {
        Self { key }
    }
}

impl SubCommand for List {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightCyan }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if args.clone().free()?.is_empty() {
            return Self::init(key).prompt_value()
        }
        println!("{}", format!("ls: {:#?}", key).color(List::color()));
        println!("{:#?}", args);
        Ok(Self { key })
    }

}

impl Default for List {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        "list".to_string()
    }
}
