
use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug)]
pub struct Link {
    key: String,
}

impl SubCommand for Link {

    fn new(key: String) -> Self {
        Self { key }
    }

    fn insert(key: String, val: String) -> Result<Self, pico_args::Error> {
        Ok(Self::default())
    }

    fn color() -> Color { Color::BrightYellow }

    fn with_args(key: String, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        if args.clone().free()?.is_empty() {
            return Self::new(key).prompt_value();
        }
        println!("{}", format!("L: {:#?}", key).color(Link::color()));
        println!("{:#?}", args);
        Ok(Self { key })
    }

}

impl Default for Link {
    fn default() -> Self {
        Self::prompt_key().unwrap()
    }
}

impl ToString for Link {
    fn to_string(&self) -> String {
        "link".to_string()
    }
}


