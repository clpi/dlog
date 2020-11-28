
use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Link {
    key: String,
    val: Option<String>
}



impl SubCommand for Link {

    fn cmd_string() -> Vec<&'static str> {
        vec!["link", "l"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key, val}
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightYellow }

}

impl Default for Link {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        let new = Self::new(key, Some(val));
        new
    }
}

impl ToString for Link {
    fn to_string(&self) -> String {
        "link".to_string()
    }
}


