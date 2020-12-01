use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::{SubCommand, DataType};
use chrono::{DateTime, Utc};
use crate::cmd::Item;

#[derive(Debug,Clone)]
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

    fn with_args(mut key: Option<String>, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        match (key, args.subcommand()?) {
            (Some(mut key), Some(mut val)) => {
                if Self::cmd_string().contains(&val.as_str()) {
                    key  = val;
                    return Self::with_args(Some(key.clone()), args);
                }
                println!("{}", format!("{}: {:?}, Item: {:?}",
                        Self::cmd_string()[0], key, val)
                    .color(Self::color()));
                let _item = Item::with_args(Some(val.clone()), args)?;
                let record = Self::new(key, Some(val));
                record.insert()?;
                Ok(record)
            }
            (Some(mut key), None) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    key = Self::prompt_key()?;
                }
                let val = Self::new(key.clone(), None).prompt_value()?;
                println!("{}", format!("{}: {:?}, Field: {}",
                        Self::cmd_string()[0], key, val.clone())
                    .color(Self::color()));
                let _item = Item::with_args(Some(val.clone()), args)?;
                let record = Self::new(key, Some(val));
                record.insert()?;
                Ok(record)
            }
            _ => {
                let record = Self::default();
                let _field = Item::with_args(record.clone().val, args)?;
                Ok(record)
            }
        }
    }

    fn kind() -> String { "record".into() }

}

impl DataType for Record {}

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
        self.key.to_owned()
    }
}

