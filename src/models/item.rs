use std::{rc::Rc, path::PathBuf};
use chrono::{prelude::*, Utc, DateTime};
use serde::{Serialize, Deserialize};
use colored::{Color, Colorize};
use crate::{
    prompt::prompt,
    error::DResult,
    models::{Entry, Fact, Record, Attrib, Note},
};
use uuid::Uuid;
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "Id")]
    pub id: uuid::Uuid,
    #[serde(rename = "Item")]
    pub name: String,
    #[serde(rename = "Attributes")]
    pub attribs: Vec<Attrib>,
    #[serde(rename = "Notes")]
    pub notes: Vec<Note>,
    #[serde(rename = "Created at")]
    pub created: DateTime<Local>,
}

impl Default for Item {
    fn default() -> Self {
        // TODO make this into a function called by all default functions
        let name = prompt("Item name: ")
            .expect("Could not prompt item name");
        println!("{}", format!("Got new item: {}", &name)
            .color(Color::BrightCyan));
        Item {
            id: Uuid::new_v4(),
            name,
            created: Local::now(),
            notes: Vec::new(),
            attribs: Vec::new()
        }
    }
}

impl Item {

    pub fn new(name: String,) -> Self {
        let id: Uuid = Uuid::new_v4();
        Self { id, name,
                created: Local::now(),
                notes: Vec::new(),
                attribs: Vec::new(),
        }
    }

    pub fn insert(&self, record: Record) -> std::io::Result<PathBuf> {
        let item = record.add_item(self)?;
        Ok(item)
    }

    pub fn get_all_facts(&self) -> DResult<Vec<Fact>> {
        let mut facts: Vec<Fact> = vec![];
        let mut rdr = csv::Reader::from_path("~/test.csv")?;
        while let Some(rec) = rdr.records().next() {
            let fact: Fact = rec?.clone().deserialize(None)?;
            facts.push(fact);
        }
        Ok(facts)
    }

    pub fn get_fact(&self, fact: &str) -> DResult<Vec<Fact>> {
        let mut facts: Vec<Fact> = vec![];
        let mut rdr = csv::Reader::from_path("~/test.csv")?;
        while let Some(rec) = rdr.records().next() {
            let rec = rec?.clone();
            let name = &rec[0];
            if name.eq_ignore_ascii_case(fact) {
                let fact = rec.deserialize(None)?;
                facts.push(fact);
            }
        }
        Ok(facts)

    }

}

impl FromArgMatches for Item {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match (matches.value_of("NAME"), matches.value_of("record")) {
            (Some(item), Some(record)) => {
                Self::new(item.into())
            },
            (Some(item), None)  => {
                Self::new(item.into())
            },
            (_, _) => Self::default(),
        }
    }
}


// TODO add fact links as rows in table and add notes
impl comfy_table::ToRow for Item {
    fn to_row(self) -> comfy_table::Row {
        comfy_table::Row::from(vec![
            &self.id.to_string(),
            &self.name.to_string(),
            &Attrib::join(&self.attribs),
            &Note::join(&self.notes),
            &self.created.to_string(),
        ])
    }
}

impl Entry for Item {
    fn datetime(&self) -> DateTime<Local> {
        self.created
    }
}
