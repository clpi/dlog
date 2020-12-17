use crate::{
    csv as Csv, util::prompt_input,
    models::{
        units::Units,
        record::Record,
        item::Item,
        attrib::Attrib,
    },
};
use uuid::Uuid;
use std::{fmt, path::PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

/// A single key-value pair to be logged into a csv corresponding to the fact's
/// name (key). Fact entries are automatically tagged with their time of entry
/// and each entry may optionally be associated with a number of different attributes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Value")]
    pub val: String,
    #[serde(rename="Units")]
    pub unit: Units,
    #[serde(rename="Datetime")]
    pub time: DateTime<Utc>,
    #[serde(rename="Attribute")]
    pub attribs: Vec<Attrib>,
}

impl Fact {

    pub fn new(
        name: String,
        val: String,
        unit: Units,
        attribs: Vec<Attrib>) -> Self
    {
        let unit = Units::from(unit);
        Self {
            id: Uuid::new_v4(),
            name, val, time: Utc::now(), unit, attribs
        }
    }

    pub fn write(
        &self,
        record: Option<Record>,
        item: Option<Item>
    ) -> std::io::Result<()>
    {
        match (record, item) {
            (Some(record), Some(item)) => {
                let rec = record.get_or_create()?;
                let rec = rec.parent().expect("Could not get record parent");
                let item = PathBuf::from(rec).join(item.name);
                let mut wtr = Csv::csv_writer(item, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;

            },
            (Some(record), None) => {
                let mut wtr = Csv::csv_writer(record.get_or_create()?, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?
            },
            (None, Some(item)) => {
                // TODO put in uncategorized, don't prompt for record
                let rec = Record::from(prompt_input("What is the record name?: ")?);
                let rec = rec.get_or_create()?;
                let rec = rec.parent().expect("Could not get record parent");
                let item = PathBuf::from(rec).join(item.name);
                let mut wtr = Csv::csv_writer(item, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;
            }
            (None, None) => {
                let rec = Record::default().get_or_create()?;
                let mut wtr = Csv::csv_writer(rec, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;
            }
        };
        Ok(())
    }

    pub fn parse_units_in_val(_val: String) -> Option<(String, String)> {
        None
    }

    pub fn parse_date(val: String) -> chrono_english::DateResult<String> {
        // use chrono_english::{parse_date_string, Dialect};
        // use chrono::prelude::*;
        Ok(String::new())
    }
}

impl Default for Fact {
    fn default() -> Self {
        let name = prompt_input("Fact name: ")
            .expect("Could not prompt fact name");
        let val = prompt_input("Fact value: ")
            .expect("Could not prompt fact value");
        let unit = Units::prompt("Units? (Enter if not applicable): ");
        let attribs = Attrib::prompt("Attributes? (Enter if not applicable): ");
        println!("{}", format!("Got new fact: {} = {} {:?}, attribs {:?}",
                &name, &val, &unit, &attribs).color(Color::BrightCyan));
        Fact::new(name, val, unit, attribs)
    }
}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attribs = &self.clone().attribs;
        let attribs: String = Attrib::join(attribs);
        f.write_fmt(format_args!("Fact: {}: {} {} {}",
            &self.name, &self.val, &self.unit, &attribs))
    }
}
