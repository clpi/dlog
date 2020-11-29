use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};
use crate::cmd::Fact;

/// Items are the main object of interest in dlog. By default, an ambiguous
/// subcommand always defaults to the assumption of an item name
#[derive(Debug, Clone,)]
pub struct Item {
    key: String,
    val: Option<String>,
    created: DateTime<Utc>,
    Facts: Vec<Fact>,
}

impl Item {

}

impl SubCommand for Item {

    fn cmd_string() -> Vec<&'static str> {
        vec!["item", "i", "-i", "--item"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self { key, val, created: Utc::now(), Facts: vec![] }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightYellow }

    fn with_args(key: Option<String>, args: &mut Arguments) -> Result<Self, pico_args::Error> {
        match (key, args.subcommand()?) {
            (Some(mut key), Some(val)) => {
                if Self::cmd_string().contains(&val.as_str()) {
                    key  = val;
                    return Self::with_args(Some(key.clone()), args);
                }
                println!("{}", format!("{}: {:?}, Fact: {:?}",
                        Self::cmd_string()[0], key, val)
                    .color(Self::color()));
                let _Fact= Fact::with_args(Some(val.clone()), args)?;
                let item = Self::new(key, Some(val));
                item.insert()?;
                Ok(item)
            }
            (Some(mut key), None) => {
                if Self::cmd_string().contains(&key.as_str()) {
                    key = Self::prompt_key()?;
                }
                let val = Self::new(key.clone(), None).prompt_value()?;
                println!("{}", format!("{}: {:?}, Fact: {}",
                        Self::cmd_string()[0], key, val.clone())
                    .color(Self::color()));
                let Fact = Fact::with_args(Some(val.clone()), args)?;
                let item = Self::new(key.clone(), Some(val.clone()));
                item.insert()?;
                Self::show_in_table(vec![
                    vec![
                    Self::cmd_string()[0],
                    key.clone().as_str(),
                    val.clone().as_str(),
                    Fact.val.unwrap().as_str()
                    ]
                ], vec!["Type".into(), "Name".into(), "Fact".into(), "Value".into()]);
                Ok(item)
            }
            _ => {
                let item = Self::default();
                let _fact = Fact::with_args(item.clone().val, args)?;
                Ok(item)
            }
        }
    }

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
