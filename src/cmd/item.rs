use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};

#[derive(Debug,)]
pub struct Item {
    key: String,
    val: Option<String>,
    created: DateTime<Utc>,
}

impl Item {

}

impl SubCommand for Item {

    fn cmd_string() -> Vec<&'static str> {
        vec!["item", "i"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key, val, created: Utc::now() }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightYellow }

}

impl Default for Item {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        Self::new(key, Some(val))
    }
}

impl ToString for Item {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
