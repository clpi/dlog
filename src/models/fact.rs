use crate::{
    csv as Csv, prompt,
    models::{
        Entry,
        units::Units,
        record::Record,
        item::Item,
        note::Notes,
        attrib::Attrib,
    },
};
use uuid::Uuid;
use std::{fmt, path::PathBuf, collections::HashMap};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

// TODO add units
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AbstractFact {
    #[serde(rename="Id", default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes", default="Vec::new")]
    pub notes: Vec<Notes>,
    #[serde(rename="Datetime", default="Local::now")]
    pub created_at: DateTime<Local>
}

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
    #[serde(rename="Units", default="Units::default")]
    pub unit: Units,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes", default="Vec::new")]
    pub notes: Vec<Notes>,
    #[serde(rename="Datetime", default="Local::now")]
    pub created_at: DateTime<chrono::Local>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FactValue {
    Integer(i32),
    RealNumber(f32),
    Option(HashMap<String, bool>), //TODO find a way to parse this
    ExactTime(DateTime<Local>),
    Duration(std::time::Duration),
    Day(DateTime<Local>),
    Boolean(bool),
    Text(String),
    Range(f32, f32),
}

impl Default for FactValue {
    fn default() -> Self {
        FactValue::Boolean(true)
    }
}

impl From<String> for FactValue {
    fn from(val: String) -> Self {
        if let Ok(num) = val.parse::<f32>() {
            FactValue::RealNumber(num)
        } else if let Ok(num) = val.parse::<i32>() {
            FactValue::Integer(num)
        } else if val.contains("-") { // | val.contains("to")?
            let pair = val.split("-").take(2).collect::<Vec<&str>>();
            if let (Ok(n1), Ok(n2)) = (pair[0].parse::<f32>(), pair[1].parse::<f32>()) {
                FactValue::Range(n1, n2)
            } else { FactValue::Text(val) }
        } else {
            match val.as_str() {
                "true" | "yes" | "t" | "y" => FactValue::Boolean(true),
                "false" | "no" | "f" | "n" => FactValue::Boolean(false),
                _ => FactValue::Text(val.into()),
            }
        }
    }
}

impl fmt::Display for FactValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactValue::Text(txt) => f.write_fmt(format_args!("Text val: {}", txt)),
            FactValue::Boolean(b) => f.write_fmt(format_args!("Bool value: {}", b)),
            FactValue::RealNumber(r) => f.write_fmt(format_args!("Real num: {}", r)),
            FactValue::Integer(i) => f.write_fmt(format_args!("Integer: {}", i)),
            FactValue::Range(n1, n2) => {
                f.write_fmt(format_args!("Range value from {} to {}",n1, n2))
            },
            FactValue::Option(map) => {
                let mut out = String::new();
                for (opt, sel) in map.iter() {
                    if *sel {
                        out.push_str(&format!("Val opt {} was selected", opt));
                    } else {
                        out.push_str(&format!("Val opt {} not selected", opt));
                    }
                }
                f.write_str(out.as_str())
            },
            _ => { f.write_str("Other") }
        }
    }
}

impl Fact {

    pub fn new(
        name: String,
        val: String,
        unit: Units,
        attribs: Vec<Attrib>,
        notes: Vec<Notes>) -> Self
    {
        let unit = Units::from(unit);
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
        let attribs = self.clone().attribs;
        let notes = self.clone().notes;
        f.write_fmt(format_args!("Fact: {}: {} {:#?} {:#?}",
            self.id, self.name, attribs, notes))
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
            unit: Units::Other(rec[4].to_string()), //TODO handle date parsing
            attribs: rec.iter().skip(5)
                .map(|a| Attrib::new(a, None))
                .collect(),
            notes: rec.iter().skip(6)
                .map(|n| Notes::new(n))
                .collect(),
        };
        Ok(fact)
    }
}


impl Entry for Fact {
    fn datetime(&self) -> chrono::DateTime<chrono::Local> {
        self.created_at
    }
}

impl FromArgMatches for Fact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        let name = if let Some(name) = matches.value_of("NAME") {
            name.to_string()
        } else {
            crate::prompt::prompt("Fact name?: ").unwrap().to_string()
        };
        if let Some(value) = matches.value_of("VALUE") {
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
                .map(|n| { println!(" note: {:?}", n); Notes::new(n) })
                .collect::<Vec<Notes>>();
            Self::new(name.into(), value.into(), units, attr, notes)
        } else {
            Self { name: name.into(),
                val: FactValue::Boolean(true), // NOTE val "true" if no val specified
                ..Self::default()
            }
        }
    }
}

// TODO Add logic that makes it such that a previously created abstract fact
//      (i.e. a fact entry with the key has already been made) will simply
//      lookup the corresponding abstract fact
impl FromArgMatches for AbstractFact {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        let name = if let Some(name) = matches.value_of("NAME") {
            name.to_string()
        } else {
            crate::prompt::prompt("Fact name?: ").unwrap().to_string()
        };
        let id = uuid::Uuid::new_v4();
        let linked_attribs = matches.values_of("link-attrib")
            .unwrap_or_default()
            .map(|att| {
                let att = Attrib::from(att.to_string());
                println!("linked att: {:?}", att);
                att})
            .collect::<Vec<Attrib>>();
        let linked_notes = matches.values_of("link-notes")
            .unwrap_or_default()
            .map(|note| {
                let note = Notes::new(note);
                println!(" linked note {:?}", note);
                note
            })
            .collect::<Vec<Notes>>();
        Self { id, name, attribs: linked_attribs, notes: linked_notes, created_at: Local::now() }
    }
}
