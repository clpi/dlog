use std::path::PathBuf;
use sled::{Config as SConfig, Db, IVec};
use byteorder::{BigEndian, LittleEndian};
use zerocopy::{
        byteorder::U64, AsBytes, FromBytes, LayoutVerified, Unaligned, U16, U32,
};
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

    pub fn new() -> sled::Result<Self> {
        let db = sled::open("dlog_db")?;
        let data = crate::config::DConfig::data_dir();
        let loc = data.join("ddb");
        Ok(Self { db, loc })
    }

    pub fn init(&self) -> sled::Result<()> {
        let _reco = self.db.open_tree("record")?;
        let _item = self.db.open_tree("item")?;
        Ok(())
    }

    pub fn insert_fact(&self, fact: Fact) -> sled::Result<()> {
        let _fact = self.db.open_tree("fact")?;
        // fact.insert(fact.id, serde:)
        Ok(())
    }

    pub fn upsert(&self) -> sled::Result<()> {
        #[derive(FromBytes, AsBytes, Unaligned)]
        #[repr(C)]
        struct Key {
            a: U64<BigEndian>,
            b: U64<BigEndian>
        }
        #[derive(FromBytes, AsBytes, Unaligned)]
        #[repr(C)]
        struct Value {
            count: U64<LittleEndian>,
            whatever: [u8; 16],
        }

        let key = Key { a: U64::new(21), b: U64::new(890) };
        self.db.update_and_fetch(key.as_bytes(), |value_opt| {
            if let Some(existing) = value_opt {
            let mut backing_bytes = sled::IVec::from(existing);
            let layout: LayoutVerified<&mut [u8], Value> =
                LayoutVerified::new_unaligned(&mut *backing_bytes)
                    .expect("bytes do not fit schema");
            let value: &mut Value = layout.into_mut();
            let new_count = value.count.get() + 1;
            println!("incrementing count to {}", new_count);
            value.count.set(new_count);
            Some(backing_bytes)
        } else {
            println!("setting count to 0");
            Some(sled::IVec::from(
                Value { count: U64::new(0), whatever: [0; 16] }.as_bytes(),
            ))
        }
        })?;

        Ok(())
    }

    fn export(&self) -> Vec<(Vec<u8>, Vec<u8>, impl Iterator<Item=Vec<Vec<u8>>>)> {
        self.db.export()
    }

    fn remove(&self, kind: &str, name: &str) -> Option<IVec> {
        self.db.remove(name).unwrap_or_default()
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

pub trait Insertable: Sized {

    fn insert(db: &sled::Db) -> sled::Result<()> {
        Ok(())
    }

    fn get(db: &sled::Db) -> sled::Result<Self>;

    fn delete(db: &sled::Db) -> sled::Result<()> {
        Ok(())
    }
}

