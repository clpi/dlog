use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};

#[derive(Debug,)]
pub struct Record {
    key: String,
    val: Option<String>,
    created: DateTime<Utc>,
}

impl Record {

    pub fn self_sans_key() -> Self {
        Self {
            key: "Uncategorized".into(),
            created: Utc::now(),
            val: None,
        }
    }

}

impl SubCommand for Record {

    fn cmd_string() -> Vec<&'static str> {
        vec!["record", "rec", "r"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key , val, ..Self::default()}
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightCyan }

}

impl Default for Record {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        let new = Self::new(key, Some(val));
        new
    }
}

// TODO list items and fields
impl ToString for Record {
    fn to_string(&self) -> String {
        "record".to_string()
    }
}

