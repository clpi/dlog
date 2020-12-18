pub mod action;
pub mod fact;
pub mod graph;
pub mod attrib;
pub mod item;
pub mod record;
pub mod units;
pub mod stats;
pub mod value;

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

pub trait Entry {

    fn datetime(&self) -> chrono::DateTime<chrono::Utc>;

    fn date(&self) -> chrono::Date<chrono::Utc> {
        self.datetime().date()
    }

    fn time(&self) -> chrono::NaiveTime {
        self.datetime().time()
    }

    fn weekday(&self) -> chrono::Weekday {
        self.datetime().weekday()
    }
}
