use comfy_table::{
    Table, ContentArrangement, presets,
    Cell, Attribute, Color as TColor, ToRow,
};
use crate::{
    csv as Csv, prompt,
    models::{
        Entry,
        fact::{FactValue, AbstractFact, Unit, UserUnit},
        record::Record,
        item::Item,
        note::{Note, Notes},
        attrib::{Attrib, Attribs},
        date::{Datelike, Duration, RelativeTo, Recurring},
    },
};
use uuid::Uuid;
use std::{convert::TryFrom, fmt, path::PathBuf, collections::HashMap, str::FromStr};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

/// A single key-value pair to be logged into a csv corresponding to the fact's
/// name (key). Fact entries are automatically tagged with their time of entry
/// and each entry may optionally be associated with a number of different attributes.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Fact {
    #[serde(rename="Id", default="Uuid::new_v4")]
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Value")]
    pub val: FactValue,
    #[serde(rename="Units", default="Unit::default")]
    pub unit: Unit,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes", default="Vec::new")]
    pub notes: Vec<Note>,
    #[serde(rename="Datetime", default="Local::now")]
    pub created_at: DateTime<chrono::Local>,
}

impl Fact {

    pub fn new(
        name: String,
        val: String,
        unit: Unit,
        attribs: Vec<Attrib>,
        notes: Vec<Note>) -> Self
    {
        let unit = Unit::from(unit);
        let val = FactValue::from(val);
        // TODO parse val into appropriate fact value
        Self {
            id: Uuid::new_v4(),
            name, val, created_at: Local::now(), unit, attribs, notes,
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
                let mut wtr = crate::csv::csv_writer(item, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;

            },
            (Some(record), None) => {
                let mut wtr = crate::csv::csv_writer(record.get_or_create()?, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?
            },
            (None, Some(item)) => {
                // TODO put in uncategorized, don't prompt for record
                let rec = Record::from(prompt::prompt("What is the record name?: ")?);
                let rec = rec.get_or_create()?;
                let rec = rec.parent().expect("Could not get record parent");
                let item = PathBuf::from(rec).join(item.name);
                let mut wtr = crate::csv::csv_writer(item, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;
            }
            (None, None) => {
                let rec = Record::default().get_or_create()?;
                let mut wtr = crate::csv::csv_writer(rec, self)?;
                wtr.serialize(self).expect("Could not serialize fact");
                wtr.flush()?;
            }
        };
        Ok(())
    }

    pub fn parse_tags(self) -> Option<Vec<String>> {
        let mut tags: Vec<String> = Vec::new();
        match self.val {
            FactValue::Text(txt) => {
                tags.extend_from_slice(txt.split_whitespace()
                    .filter(|w| w.starts_with("#"))
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>().as_slice());
                Some(tags)
            },
            _ => None,
        }
    }

    pub fn parse_units_in_val(_val: String) -> Option<(String, String)> {
        None
    }

    pub fn parse_date(val: String) -> chrono_english::DateResult<String> {
        // use chrono_english::{parse_date_string, Dialect};
        // use chrono::prelude::*;
        Ok(String::new())
    }

    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table.load_preset(presets::UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Fact").add_attribute(Attribute::Bold)
                    .fg(TColor::Blue),
                Cell::new("Value").add_attribute(Attribute::Bold),
                Cell::new("Units").add_attribute(Attribute::Bold),
                Cell::new("Attributes").add_attribute(Attribute::Bold),
                Cell::new("Notes").add_attribute(Attribute::Bold),
                Cell::new("Created").add_attribute(Attribute::Bold),
            ])
            .add_row(vec![
                &self.name.to_string(),
                &self.val.to_string(),
                &self.unit.to_string(),
                &Attrib::join(&self.attribs),
                &Note::join(&self.notes),
                &self.created_at.to_string(),
            ]);
        table
        }

}
impl Default for Fact {
    fn default() -> Self {
        let name = prompt::prompt("Fact name?").unwrap();
        let val = prompt::prompt("Value name?").unwrap();
        // if config.propt_units {}
        let unit = Unit::prompt("Units? (Enter if not applicable): ");
        // if config.prompt.attribs {}
        let attribs = Attrib::prompt("Attributes? (Enter if not applicable): ");
        let notes = Note::prompt("fact", name.as_str()).unwrap();
        // println!("{}", format!("Got new fact: {} = {} {:?}, attribs {:?}",
                // &name, &val, &unit, &attribs).color(Color::BrightCyan));
        let ab = AbstractFact { name: name.clone(), ..Default::default() };
        ab.insert().expect("Could not insert fact type");
        Fact::new(name, val, unit, attribs, vec![notes])
    }
}


impl comfy_table::ToRow for Fact {
    fn to_row(self) -> comfy_table::Row {
        comfy_table::Row::from(vec![
            &self.name.to_string(),
            &self.val.to_string(),
            &self.unit.to_string(),
            &Attrib::join(&self.attribs),
            &Note::join(&self.notes),
            &self.created_at.to_string(),
        ])
    }
}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.table()))
    }
}
impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        let name = if let Some(name) = matches.value_of("NAME") {
            name.to_string()
        } else {
            crate::prompt::prompt("NONAMEGIVEN: Fact name?: ").unwrap().to_string()
        };
        let attribs = Attrib::get_matches(&matches);
        let notes = Note::get_matches(&matches);
        let val = FactValue::from_arg_matches(&matches);
        let unit = Unit::from_match(matches.values_of("UNIT"));
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: Local::now(),
            name, notes, val, unit, attribs,
        }
    }
}

impl Entry for Fact {

    fn datetime(&self) -> chrono::DateTime<chrono::Local> {
        self.created_at
    }

}
impl std::convert::TryFrom<csv::StringRecord> for Fact {
    type Error = csv::Error;
    fn try_from(rec: csv::StringRecord) -> Result<Self, Self::Error> {
        let fact = Fact {
            id: Uuid::parse_str(&rec[0])
                .expect("Could not parse UUID"),
            name: rec[1].to_string(),
            val: FactValue::from(rec[1].to_string()),
            created_at: DateTime::parse_from_rfc2822(&rec[3])
                .expect("Could not parse datetime").into(),
            unit: Unit::Other(UserUnit::from(rec[4].to_string())), //TODO handle date parsing
            attribs: Attrib::from_col(&rec, 5),
            notes: Note::from_col(&rec, 6),
        };
        Ok(fact)
    }
}
