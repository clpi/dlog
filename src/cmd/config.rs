use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Config {
    key: String,
    val: Option<String>,
    created: DateTime<Utc>,
}

impl Config {

    pub fn init(key: String, val: Option<String>) -> Self {
        Self { key, val, created: Utc::now() }
    }
}

impl SubCommand for Config {

    fn cmd_string() -> Vec<&'static str> {
        vec!["config", "c"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key, val, created: Utc::now() }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightBlue }

    fn kind() -> String { "config".into() }
}

impl Default for Config {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        Self::new(key, Some(val))
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        self.key.to_owned()
    }
}
