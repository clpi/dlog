use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::util::prompt_input;
use colored::{Color, Colorize};
use super::{
    record::Record,
};
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Item")]
    pub name: String,
    pub record: Record,
}

impl Default for Item {
    fn default() -> Self {
        // TODO make this into a function called by all default functions
        let name = prompt_input("Item name: ")
            .expect("Could not prompt item name");
        println!("{}", format!("Got new item: {}", &name)
            .color(Color::BrightCyan));
        Item { name, record: Record::default() }
    }
}

impl Item {

    pub fn new(name: String, record: Option<String>) -> Self {
        if let Some(record) = record {
            Self { name, record: Record::from(record) }
        } else {
            Self { name, record: Record::default() }
        }
    }

    pub fn create(&self) -> std::io::Result<PathBuf> {
        let item = self.record.add_item(self)?;
        Ok(item)
    }

}

impl FromArgMatches for Item {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match (matches.value_of("NAME"), matches.value_of("record")) {
            (Some(item), Some(record)) => {
                Self {
                    name: item.into(),
                    record: Record::from(record.to_string())
                }
            },
            (Some(item), None)  => {
                Self {
                    name: item.into(),
                    record: Record::default()
                }
            },
            (_, _) => Self::default(),
        }
    }
}
