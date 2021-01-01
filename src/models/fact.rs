use crate::{
    csv as Csv, prompt,
    models::{
        Entry,
        units::Units,
        record::Record,
        item::Item,
        attrib::Attrib,
    },
};
use uuid::Uuid;
use std::{fmt, path::PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime, Local};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

pub type FactKey = String;
pub type FactVal = String;

// TODO add units
pub struct AbstractFact {
    pub name: FactKey,
    pub attribs: Vec<Attrib>,
    pub notes: Vec<String>,
}

/// A single key-value pair to be logged into a csv corresponding to the fact's
/// name (key). Fact entries are automatically tagged with their time of entry
/// and each entry may optionally be associated with a number of different attributes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Fact {
    #[serde(rename="Id", default="Uuid::new_v4")]
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: FactKey,
    #[serde(rename="Value")]
    pub val: FactVal,
    #[serde(rename="Units", default="Units::default")]
    pub unit: Units,
    #[serde(rename="Datetime", default="Local::now")]
    pub time: DateTime<chrono::Local>,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes")]
    pub notes: Vec<String>,
}

impl Fact {

    pub fn new(
        name: String,
        val: String,
        unit: Units,
        attribs: Vec<Attrib>,
        notes: Vec<String>) -> Self
    {
        let unit = Units::from(unit);
        Self {
            id: Uuid::new_v4(),
            name, val, time: Local::now(), unit, attribs, notes,
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
                let rec = Record::from(prompt::prompt("What is the record name?: ")?);
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
        let name = prompt::prompt("Fact name: ")
            .expect("Could not prompt fact name");
        let val = prompt::prompt("Fact value: ")
            .expect("Could not prompt fact value");
        let unit = Units::prompt("Units? (Enter if not applicable): ");
        let attribs = Attrib::prompt("Attributes? (Enter if not applicable): ");
        println!("{}", format!("Got new fact: {} = {} {:?}, attribs {:?}",
                &name, &val, &unit, &attribs).color(Color::BrightCyan));
        Fact::new(name, val, unit, attribs, Vec::new())
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

impl fmt::Display for AbstractFact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attribs = &self.clone().attribs;
        let notes = &self.clone().notes;
        f.write_fmt(format_args!("Fact: {}: {} {} {}",
            self.name, attribs, notes))
    }
}

impl std::convert::TryFrom<csv::StringRecord> for Fact {
    type Error = csv::Error;
    fn try_from(rec: csv::StringRecord) -> Result<Self, Self::Error> {
        let fact = Fact {
            id: Uuid::parse_str(&rec[0])
                .expect("Could not parse UUID"),
            name: rec[1].to_string(),
            val: rec[2].to_string(),
            time: DateTime::parse_from_rfc2822(&rec[3])
                .expect("Could not parse datetime").into(),
            unit: Units::Other(rec[4].to_string()), //TODO handle date parsing
            notes: if !rec[5].len().eq(&0) {
                    vec![rec[5].to_string()]
                } else {Vec::new()},
            attribs: rec.iter().skip(5)
                .map(|a| Attrib::new(a, None))
                .collect(),
        };
        Ok(fact)
    }
}


impl Entry for Fact {
    fn datetime(&self) -> chrono::DateTime<chrono::Local> {
        self.time
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        let name = if let Some(name) = matches.value_of("NAME") {
            name.to_string()
        } else {
            crate::prompt::prompt("Fact name?: ").unwrap().to_string()
        };
        println!("Got new fact: {}", &name);
        if let Some(value) = matches.value_of("VALUE") {
            println!("Got new fact: {} = {}", &name, &value);
            let units: Units = if let Some(units)
                = matches.values_of("UNIT")
            {
                if matches.occurrences_of("UNIT") == 1 {
                    Units::Other(units.take(0).collect())
                } else {
                    let units = units.into_iter().collect();
                    Units::Other(units)
                }
            } else { Units::None };
            println!("Got new fact: {} = {} ({})", &name, &value, &units);
            let attr = matches.values_of("attribs")
                .unwrap_or_default()
                .map(|att| {let att = Attrib::from(att.to_string()); println!("att: {:?}", att); att})
                .collect::<Vec<Attrib>>();
            let notes = matches.values_of("notes")
                .unwrap_or_default()
                .map(|att| { println!(" note: {:?}", att); att.to_string() })
                .collect::<Vec<String>>();
            Self::new(name.into(), value.into(), units, attr, notes)
        } else {
            Self { name: name.into(), ..Self::default()  }
        }
    }
}

impl FromArgMatches for AbstractFact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        let name = if let Some(name) = matches.value_of("NAME") {
            name.to_string()
        } else {
            crate::prompt::prompt("Fact name?: ").unwrap().to_string()
        };
        let linked_attribs = matches.values_of("link-attrib")
            .unwrap_or_default()
            .map(|att| {let att = Attrib::from(att.to_string()); println!("linked att: {:?}", att); att})
            .collect::<Vec<Attrib>>();
        let notes = matches.values_of("link-notes")
            .unwrap_or_default()
            .map(|note| { println!(" linked note {:?}", note); note.to_string() })
            .collect::<Vec<String>>();
        Self { name, attribs: linked_attribs, notes }
    }
}
