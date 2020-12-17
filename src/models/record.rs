use std::{path::PathBuf, fs};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::{
    util,
    models::{Units, Fact, Item},
    cmd::{ Cmd,
    }
};
use clap::{ArgMatches, FromArgMatches, Subcommand};
use colored::{Color, Colorize, Style, Styles};
use super::attrib::Attrib;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: uuid::Uuid,
    #[serde(rename="Record")]
    pub name: String,
    #[serde(rename="Item")]
    pub items: Vec<Item>,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Inbox".into() ,
            items: Vec::new(),
        }
    }
}

impl Record {

    pub fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            Self { id: Uuid::new_v4(), name, items: Vec::new() }
        } else {
            Self::default()
        }
    }

    pub fn get_or_create(&self) -> std::io::Result<PathBuf> {
        let rec_dir = util::get_or_create_data_dir()?
            .join(&self.name);
        if rec_dir.exists() && rec_dir.is_dir() {
            let rec = rec_dir.join(&format!("{}.csv", &self.name));
            if rec.exists() && rec.is_file() {
                return Ok(rec)
            } else {
                fs::File::create(&rec)?;
                return Ok(rec)
            }
        } else {
            fs::create_dir(&rec_dir)?;
            let rec = rec_dir.join(&format!("{}.csv", &self.name));
            fs::File::create(&rec)?;
            Ok(rec)
        }
    }

    pub fn add_item(&self, item: &Item) -> std::io::Result<PathBuf> {
        let rec = self.get_or_create()?;
        let item = rec.parent().expect("Could not find parent")
            .join(format!("{}{}", &item.name, ".csv"));
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(true)
            .flexible(true)
            .from_path(&item)?;
        wtr.flush()?;
        Ok(item)
    }

    pub fn read(&self, item: Option<String>) -> std::io::Result<Vec<Fact>> {
        let rec = self.get_or_create()?;
        let csv = if let Some(item) = item {
            PathBuf::from(rec).join(&format!("{}.csv", &item))
        } else {
            rec
        };
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(csv::Trim::All)
            .double_quote(false)
            .escape(Some(b'\\'))

            .from_path(&csv)?;
        let _headers = rdr.headers()?.clone();
        while let Some(rec) = rdr.records().next() {
            match rec {
                Ok(rec) => {
                    println!("{:#?}", rec);
                    let attribs: Vec<Attrib> = rec.iter().skip(3)
                        .map(|a| Attrib::new(a))
                        .collect();
                    let fact = Fact {
                        id: Uuid::parse_str(&rec[0])
                            .expect("Could not parse UUID"),
                        name: rec[1].to_string(),
                        val: rec[2].to_string(),
                        time: DateTime::parse_from_rfc2822(&rec[3])
                            .expect("Could not parse datetime").into(),
                        unit: Units::Other(rec[4].to_string()), //TODO handle date parsing
                        attribs,
                    };
                    println!("{:#?}", fact);
                },
                Err(e) => return Err(From::from(e)),
            };
        }
        Ok(vec![Fact::default()])
    }

}

impl From<String> for Record {
    fn from(name: String) -> Self {
        Self { id: uuid::Uuid::new_v4(), name, items: Vec::new() }
    }
}

impl FromArgMatches for Record {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}