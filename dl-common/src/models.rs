pub mod action;
pub mod fact;
pub mod graph;
pub mod attrib;
pub mod item;
pub mod record;
pub mod units;
pub mod stats;
pub mod value;
pub mod relation;
pub mod note;

pub use note::Notes;
pub use relation::Relation;
pub use units::Units;
pub use fact::Fact;
pub use record::Record;
pub use item::Item;
pub use attrib::Attrib;
pub use graph::Graph;
pub use stats::Stats;
pub use action::Action;

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
