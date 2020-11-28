use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;

#[derive(Debug,)]
pub struct List {
    key: String,
    val: Option<String>,
}

impl List {

    pub fn init(key: String) -> Self {
        Self { key , val: None }
    }
}

impl SubCommand for List {

    fn cmd_string() -> Vec<&'static str> {
        vec!["list", "ls"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key, val }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightCyan }
}

impl Default for List {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        let new = Self::new(key, Some(val));
        new
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        "list".to_string()
    }
}
