use std::{rc::Rc, path::PathBuf};
use chrono::{prelude::*, Utc, DateTime};
use serde::{Serialize, Deserialize};
use crate::util::prompt_input;
use colored::{Color, Colorize};
use crate::{
    error::DResult,
    models::{Entry, Fact, Record},
};
use uuid::Uuid;
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: uuid::Uuid,
    #[serde(rename = "Item")]
    pub name: String,
    #[serde(skip)]
    pub record: Rc<Record>,
    pub created: DateTime<Utc>,
}

impl Default for Item {
    fn default() -> Self {
        // TODO make this into a function called by all default functions
        let name = prompt_input("Item name: ")
            .expect("Could not prompt item name");
        println!("{}", format!("Got new item: {}", &name)
            .color(Color::BrightCyan));
        Item {
            id: Uuid::new_v4(),
            name, record: Rc::new(Record::default()),
            created: Utc::now(),
        }
    }
}

impl Item {

    pub fn new(name: String, record: Option<String>) -> Self {
        let id: Uuid = Uuid::new_v4();
        if let Some(record) = record {
            Self { id, name, record: Rc::new(Record::from(record)), created: Utc::now()}
        } else {
            Self { id, name, record: Rc::new(Record::default()), created: Utc::now() }
        }
    }

    pub fn create(&self) -> std::io::Result<PathBuf> {
        let item = self.record.add_item(self)?;
        Ok(item)
    }

    // pub fn path(&self) -> PathBuf {

    // }

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
                Self::new(item.into(), Some(record.to_string()))
            },
            (Some(item), None)  => {
                Self::new(item.into(), None)
            },
            (_, _) => Self::default(),
        }
    }
}

impl Entry for Item {
    fn datetime(&self) -> DateTime<Utc> {
        self.created
    }
}
