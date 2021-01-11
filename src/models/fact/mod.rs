pub mod entry;
pub mod value;
pub mod units;

pub use self::{
    entry::Fact,
    value::FactValue,
    units::{UserUnit, Unit}
};

use comfy_table::{
    Table, ContentArrangement, presets,
    Cell, Attribute, Color as TColor, ToRow,
};
use crate::{
    csv as csv, prompt,
    models::{
        Entry,
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
// TODO add Unit
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AbstractFact {
    #[serde(rename="Id", default = "uuid::Uuid::new_v4")]
    pub id: uuid::Uuid,
    #[serde(rename="Fact")]
    pub name: String,
    #[serde(rename="Unit", default="Unit::default")]
    pub unit: Unit,
    #[serde(rename="Attribute", default="Vec::new")]
    pub attribs: Vec<Attrib>,
    #[serde(rename="Notes", default="Vec::new")]
    pub notes: Vec<Note>,
    #[serde(rename="Datetime", default="Local::now")]
    pub created_at: DateTime<Local>
}


impl Default for AbstractFact {
    fn default() -> Self {
        let name = prompt::prompt("Fact type name?").expect("Could not read prompt");
        let unit = Unit::prompt("Unit? (Enter if not applicable): ");
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
                Cell::new("Unit").add_attribute(Attribute::Bold),
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
impl fmt::Display for AbstractFact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attribs = self.clone().attribs;
        let notes = self.clone().notes;
        f.write_fmt(format_args!("Fact: {}: {} {:#?} {:#?}",
            self.id, self.name, attribs, notes))
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
        let Unit = Unit::from_match(matches.values_of("UNIT"));
        let link_Unit = Unit::from_match(matches.values_of("link-unit"));
        let attr = Attrib::get_links(&matches);
        let notes = Note::get_links(&matches);
        let Unit = if link_Unit == Unit::None { Unit } else { link_Unit };
        Self { id,
            name,
            attribs: attr,
            notes,
            unit: Unit,
            created_at: Local::now()
        }
    }
}






