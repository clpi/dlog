use comfy_table::{
    Table, ContentArrangement, presets::UTF8_BORDERS_ONLY,
    Cell, Attribute, Color as TColor,
};
use std::{
    path::PathBuf, fs,
    convert::TryFrom,
    rc::Rc,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Local};
use crate::{
    util,
    models::{Entry, Units, Item, fact::{Fact, AbstractFact}},
};
use clap::{ArgMatches, FromArgMatches};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[serde(rename="Id")]
    pub id: uuid::Uuid,
    #[serde(rename="Record")]
    pub name: String,
    #[serde(rename="Items")]
    pub items: Vec<Rc<Item>>,
    #[serde(rename="Facts")]
    pub fact_types: Vec<AbstractFact>,
    #[serde(rename="Created at")]
    pub created: DateTime<Local>,
}

impl Default for Record {
    fn default() -> Self {
        Self { //FIXME should lookup UUID and created time for Inbox record, not generate
            id: Uuid::new_v4(),
            name: "Inbox".into() ,
            items: Vec::new(),
            created: Local::now(),
            fact_types: Vec::new(),
        }
    }
}

impl Record {

    pub fn new(name: Option<String>) -> Self {
        if let Some(name) = name {
            Self {
                id: Uuid::new_v4(),
                name,
                items: Vec::new(),
                fact_types: Vec::new(),
                created: Local::now(), }
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
                    let fact = Fact::try_from(rec)?;
                    println!("{:#?}", fact);
                },
                Err(e) => return Err(From::from(e)),
            };
        }
        Ok(vec![Fact::default()])
    }

    pub fn fact_entry_table(&self) -> Table {
        let mut table = Table::new();
        // let facts = self.fact_types.iter()
        table.load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_table_width(160)
            .set_header(vec![
                Cell::new("Fact").add_attribute(Attribute::Bold)
                    .fg(TColor::Blue),
                Cell::new("Value").add_attribute(Attribute::Bold),
                Cell::new("Units").add_attribute(Attribute::Bold),
                Cell::new("Attributes").add_attribute(Attribute::Bold),
                Cell::new("Notes").add_attribute(Attribute::Bold),
                Cell::new("Created").add_attribute(Attribute::Bold),
            ]);
        // TODO make lookup algorithm to look through all facts in db
        // of types corresponding to record fact types and iter through
        // them, adding their row
        table
        }

    pub fn fact_types_table(&self) -> Table {
        let mut table = Table::new();
        table.load_preset(UTF8_BORDERS_ONLY)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_table_width(160)
            .set_header(vec![
                Cell::new("Fact").add_attribute(Attribute::Bold)
                    .fg(TColor::Blue),
                Cell::new("Value").add_attribute(Attribute::Bold),
                Cell::new("Units").add_attribute(Attribute::Bold),
                Cell::new("Attributes").add_attribute(Attribute::Bold),
                Cell::new("Notes").add_attribute(Attribute::Bold),
                Cell::new("Created").add_attribute(Attribute::Bold),
            ])
            .add_row(&self.fact_types);
        table
        }

}

// FIXME implemnt
impl From<String> for Record {
    fn from(name: String) -> Self {
        Self { id: uuid::Uuid::new_v4(), name, items: Vec::new(), created: Local::now(), fact_types: Vec::new() }
    }
}

impl FromArgMatches for Record {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::default()
    }
}

// TODO make a hierarchical table display of a table and its items/facts
// So that a record calling fmt can just call fmt on all of its facts and produce
// one big table for the record with all the facts and items
impl Entry for Record {
    fn datetime(&self) -> chrono::DateTime<Local> {
        self.created
    }

}

pub struct RecordIdKey {
    id: uuid::Uuid,
    uid: uuid::Uuid,
}

pub struct RecordNameKey {
    name: String,
    username: String,
}
