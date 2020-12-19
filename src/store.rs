use dirs_next::data_dir;
use std::{
    fs, path::PathBuf,
    sync::{Arc, Mutex},
};
use crate::{
    error::DError,
    models::Record,
    util::prompt_input,
};

#[derive(Debug)]
pub struct Store {
    loc: PathBuf,
    records: Arc<Mutex<Vec<Record>>>,
}

impl Default for Store {
    fn default() -> Self {
        let dir = if let Some(ddir) = data_dir() {
            ddir
        } else if let Ok(ddir) = prompt_input("Data dir?") {
            PathBuf::from(ddir)
        } else {
            PathBuf::default()
        };
        Self {
            loc: dir,
            records: Arc::new(Mutex::new(vec![ Record::default() ]))
        }
    }
}

impl Store {

    pub fn add_record(&mut self, record: Record) {
        if let Ok(mut rec) = self.records.lock() {
            rec.push(record);
        }
    }

    pub fn remove_record(&mut self, record: &str) -> Result<(), DError> {
        Ok(())
    }

    pub fn change_dir<D: Into<PathBuf>>(&mut self, dir: D) -> Result<(), DError> {
        Ok(())
    }

}
