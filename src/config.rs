use std::{
    fs, io, path::PathBuf
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Default, Deserialize)]
struct Config {
    name: String,
    data_loc: PathBuf,
}

impl Config {

    pub fn new() -> io::Result<Self> {
        let mut conf = fs::File::create("dlog.toml")?;
        Ok(Self::default())
    }
    pub fn load() -> Self {
        Self::default()
    }
}
