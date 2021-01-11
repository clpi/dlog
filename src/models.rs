pub mod action;
pub mod data;
pub mod fact;
pub mod graph;
pub mod attrib;
pub mod item;
pub mod record;
pub mod stats;
pub mod relation;
pub mod note;
pub mod date;
pub mod topic;

pub use note::{Note, Notes};
pub use relation::Relation;
pub use fact::{Unit, UserUnit, Fact, FactValue, AbstractFact};
pub use record::Record;
pub use item::Item;
pub use attrib::Attrib;
pub use graph::Graph;
pub use stats::Stats;
pub use action::Action;
pub use topic::Topic;

#[derive(Debug)]
pub struct Object {

}


use chrono::{prelude::*};

// TODO consider using NaiveLocal time instead of DateTime<Local>?
pub trait Entry : clap::FromArgMatches {

    fn datetime(&self) -> chrono::DateTime<chrono::Local>;

    fn date(&self) -> chrono::Date<chrono::Local> {
        self.datetime().date()
    }

    fn time(&self) -> chrono::NaiveTime {
        self.datetime().time()
    }

    fn weekday(&self) -> chrono::Weekday {
        self.datetime().weekday()
    }

}
