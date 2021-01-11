use comfy_table::{
    Table, ContentArrangement, presets::{self, UTF8_BORDERS_ONLY},
    Cell, Attribute, Color as TColor, ToRow,
};
use crate::{
    csv as Csv, prompt,
    models::{
        Entry,
        units::{Units, UserUnit},
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

// TODO add units
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AbstractFact {
    #[serde(rename="Id", default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Units", default="Units::default")]
    pub unit: Units,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes", default="Vec::new")]
    pub notes: Vec<Note>,
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
    pub notes: Vec<Note>,
    #[serde(rename="Datetime", default="Local::now")]
    pub created_at: DateTime<chrono::Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactValue {
    Integer(i32),
    RealNumber(f32),
    Option(HashMap<String, bool>), //TODO find a way to parse this
    Datelike(Datelike),
    Recurring(Recurring),
    Boolean(bool),
    Text(String),
    Range(f32, f32),
    Duration(Duration),
    UserValue(String),
    UserEnum(UserEnum),
    Amount(i32, UserObject),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserObject(String);

impl std::cmp::PartialEq for FactValue {
    fn eq(&self, other: &FactValue) -> bool {
        match (self, other) {
            (Self::Text(t1), Self::Text(t2)) => t1 == t2,
            (Self::Boolean(b1), Self::Boolean(b2)) => b1 == b2,
            (Self::Integer(i1), Self::Integer(i2)) => i1 == i2,
            (Self::Duration(d1), Self::Duration(d2)) => d1.secs.eq(&d2.secs),
            (Self::UserEnum(ue1), Self::UserEnum(ue2)) => {
                return false;
            }
            (Self::Datelike(d1), Self::Datelike(d2)) => match (d1, d2) {
                (Datelike::Day(d1), Datelike::Day(d2)) => d1.eq(&d2),
                (Datelike::Week(w1), Datelike::Week(w2)) => w1.eq(w2),
                (Datelike::Weekday(w1, RelativeTo::Now(r1)), Datelike::Weekday(w2, RelativeTo::Now(r2))) => w1.eq(&w2) && r1.eq(&r2),
                (Datelike::Month(m1, RelativeTo::Now(r1)), Datelike::Month(m2, RelativeTo::Now(r2))) => m1.eq(&m2),
                (Datelike::Year(m1), Datelike::Year(m2)) => m1.eq(m2),
                _ => false,
            },
            _ => false,
        }
    }
}

impl std::cmp::Eq for FactValue {
    fn assert_receiver_is_total_eq(&self) {
    }
}

impl std::hash::Hash for FactValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {

    }
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized, {

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEnum {
    possible_values: HashMap<FactValue, Option<Units>>,
    choice: Option<usize>,
}

impl std::str::FromStr for FactValue {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(" ") {
            if let Ok(dl) = Datelike::from_str(s) {
                return Ok(Self::Datelike(dl));
            }
            if let Ok(dur) = s.parse::<humantime::Duration>() {
                return Ok(Self::Duration(Duration::today(dur.as_secs() as u32)));
            } else if let Ok(time) = chrono_english::parse_date_string(
                s, Local::now(), chrono_english::Dialect::Us
            ) {
                return Ok(Self::Datelike(Datelike::Datetime(time)))
            } else if let Ok(num) = s.parse::<i32>() {
                return Ok(Self::Integer(num))
            } else if let Ok(num) = s.parse::<f32>() {
                return Ok(Self::RealNumber(num))
            } else if let Ok(day) = s.parse::<chrono::Weekday>() {
                return Ok(Self::Datelike(Datelike::Weekday(day, RelativeTo::Now(Local::now()))))
            } else if let Ok(month) = s.parse::<chrono::Month>() {
                return Ok(Self::Datelike(Datelike::Month(month, RelativeTo::Now(Local::now()))))
            } else if let Ok(day) = s.parse::<chrono::NaiveDate>() {
                return Ok(Self::Datelike(Datelike::Day(day)))
            } else if let Ok(b) = s.parse::<bool>() {
                return Ok(Self::Boolean(b))
            } else {
                return Ok(Self::Text(s.to_string()))
            }
        } else {
            let s_s = s.split_whitespace().collect::<Vec<&str>>();
            return Ok(Self::Text(s.to_string()))
        }
    }
}
impl FromArgMatches for FactValue {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        if let Ok(value) = matches.value_of_t::<FactValue>("VALUE") {
            value
        } else {
            Self::Boolean(true)
        }
    }
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
            FactValue::Duration(d) => write!(f, "{}", d),
            FactValue::Datelike(d) => write!(f, "Datelike {}", d),
            FactValue::Recurring(r) => write!(f, "Recurring: {} every {}", r.date, r.event),
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
        notes: Vec<Note>) -> Self
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
        let unit = Units::prompt("Units? (Enter if not applicable): ");
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

impl Default for AbstractFact {
    fn default() -> Self {
        let name = prompt::prompt("Fact type name?").expect("Could not read prompt");
        let unit = Units::prompt("Units? (Enter if not applicable): ");
        let attribs = Attrib::prompt("Attributes? (Enter if not applicable): ");
        let notes = Note::prompt("fact", name.as_str()).unwrap();
        Self {
            name,
            id: uuid::Uuid::new_v4(),
            unit,
            attribs,
            notes: vec![notes],
            created_at: Local::now(),
        }
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

impl comfy_table::ToRow for AbstractFact {
    fn to_row(self) -> comfy_table::Row {
        comfy_table::Row::from(vec![
            &self.name.to_string(),
            &self.unit.to_string(),
            &Attrib::join(&self.attribs),
            &Note::join(&self.notes),
            &self.created_at.to_string(),
        ])
    }
}

impl AbstractFact {
    pub fn table(&self) -> Table {
        let mut table = Table::new();
        table.load_preset(presets::UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Fact Type").add_attribute(Attribute::Bold)
                    .fg(TColor::Cyan),
                Cell::new("Units").add_attribute(Attribute::Bold),
                Cell::new("Attributes").add_attribute(Attribute::Bold),
                Cell::new("Notes").add_attribute(Attribute::Bold),
                Cell::new("Created").add_attribute(Attribute::Bold),
            ])
            .add_row(self.clone().to_row());
        table
    }

    pub fn insert(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

}

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.table()))
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
            unit: Units::Other(UserUnit::from(rec[4].to_string())), //TODO handle date parsing
            attribs: Attrib::from_col(&rec, 5),
            notes: Note::from_col(&rec, 6),
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
            crate::prompt::prompt("NONAMEGIVEN: Fact name?: ").unwrap().to_string()
        };
        let attribs = Attrib::get_matches(&matches);
        let notes = Note::get_matches(&matches);
        let val = FactValue::from_arg_matches(&matches);
        let unit = Units::from_match(matches.values_of("UNIT"));
        Self {
            id: uuid::Uuid::new_v4(),
            created_at: Local::now(),
            name, notes, val, unit, attribs,
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
        let units = Units::from_match(matches.values_of("UNIT"));
        let link_units = Units::from_match(matches.values_of("link-unit"));
        let attr = Attrib::get_links(&matches);
        let notes = Note::get_links(&matches);
        let units = if link_units == Units::None { units } else { link_units };
        Self { id,
            name,
            attribs: attr,
            notes,
            unit: units,
            created_at: Local::now()
        }
    }
}
