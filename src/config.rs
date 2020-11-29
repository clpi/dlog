use std::{
    io, fs,
};
use serde::{Serialize, Deserialize};

const CONFIG_PATH: &'static str = "~./.config/dlog";
const DATA_PATH: &'static str = "~/.dlog";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: String,
    user: String,
    data_dir: String,
}

impl Config {

    pub fn load(path: String) -> io::Result<Self> {
        let conf = fs::read_to_string(&path)?;
        Ok(Self { path, ..Self::default() })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { user: String::new(), ..Default::default() }
    }
}
