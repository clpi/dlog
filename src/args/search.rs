use chrono::{DateTime, Local};
use crate::models::{
    Entry, Fact, Record, Item, Attrib,
    note::Note, Action, Relation, value::FactValue, Units
};
use clap::{ FromArgMatches, ArgMatches };

#[derive(Default, Debug)]
pub struct Search {
    query_str: String,
    filters: Filters,
}

impl clap::FromArgMatches for Search {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Self {
        Search { query_str: "".into(), filters: Filters::None}
    }
}

#[derive(Debug)]
pub enum Filters {
    InItems(Vec<Item>),
    InRecord(Vec<Record>),
    WithAttribute(Vec<Attrib>),
    WithUnit(Vec<Units>),
    NotesContaining(Vec<String>),
    NameContaining(Vec<String>),
    HasValue(Vec<FactValue>),
    CreatedBefore(DateTime<Local>),
    CreatedAfter(DateTime<Local>),
    WithRelation(Vec<String>),
    None,

}

impl FromArgMatches for Filters {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::HasValue(Vec::new())
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::None
    }
}
