use crate::models::{Fact, Item, Record, Attrib, Relation};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Query {
    pub terms: Vec<String>,
    pub filters: Vec<QueryFilter>,
}

impl Query {

    pub fn show_progress() {
        let mut pb = ProgressBar::new(50);
        pb.set_message("Beginning search...");
        pb.set_style(ProgressStyle::default_bar());
        for i in 0..100 {
            pb.set_message(format!("At {} %", i).as_str());
            pb.inc(1);
        }
        pb.finish_with_message("Finished!");

    }
}

pub enum QueryFilter {
    Facts,
    Items,
    Records,
    AllLike(String),
    FactsLike(String),
    ItemsLike(String),
    RecordsLike(String),
    ItemsWithFact(Fact),
    RecordsWithFact(Fact),
    // etc etc TODO implement
}
