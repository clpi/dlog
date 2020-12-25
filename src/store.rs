use sled::{Config as SConfig, Db, IVec};
use std::path::PathBuf;
use crate::{
    config::DConfig,
    error::DError,
    models::{Record, Fact, Item},
};

#[derive(Debug)]
pub struct Store {
    db: sled::Db,
    loc: PathBuf,
}

impl Store {

    pub fn init(&self) -> sled::Result<()> {
        let reco = self.db.open_tree("record")?;
        let item = self.db.open_tree("item")?;


        Ok(())
    }

    pub fn insert_fact(&self, fact: Fact) -> sled::Result<()> {
        let fact = self.db.open_tree("fact")?;
        // fact.insert(fact.id, serde:)

        Ok(())
    }
}

impl Default for Store {
    fn default() -> Self {
        let loc = DConfig::conf_dir().join("db");
        let db = SConfig::default()
            .path(&loc)
            .cache_capacity(10_000_000)
            .use_compression(true)
            .mode(sled::Mode::HighThroughput)
            .temporary(false)
            .create_new(true)
            .open().expect("Could not open DB");
        Self {
            db: sled::open(&loc).unwrap(), loc,
        }
    }
}

impl Store {

    pub fn add_record(&mut self, record: Record) {
        // self.db.insert("record", IVec::from(record));
        // if let Ok(mut rec) = self.records.lock() {
            // rec.push(record);
        // }
    }

    pub fn remove_record(&mut self, record: &str) -> Result<(), DError> {
        Ok(())
    }

    pub fn change_dir<D: Into<PathBuf>>(&mut self, dir: D) -> Result<(), DError> {
        Ok(())
    }

}
