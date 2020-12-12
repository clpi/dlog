use dirs_next::{home_dir, config_dir};
use std::{
    fs, io::{self, prelude::*}, path::PathBuf
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FormatConfig {
    Csv,
    Json,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: Option<String>,
    format: Option<FormatConfig>,
    data_loc: Option<PathBuf>,
    record: Option<RecordConfig>,
    item: Option<ItemConfig>,
    fact: Option<FactConfig>,
    user: Option<UserConfig>,
}

impl Config {

    pub fn load() -> io::Result<Self> {
        let path = Self::conf_dir().join("dlog.toml");
        let conf = if path.exists() {
            let toml = toml::de::from_str(&fs::read_to_string(path)?)?;
            toml
        } else {
            let mut conf_file = fs::File::create(path)?;
            let config = Self::default();
            let toml = toml::ser::to_string_pretty(&config)
                .expect("Could not serialize config to TOML");
            conf_file.write_all(toml.as_bytes())?;
            config
        };
        Ok(conf)
    }

    pub fn conf_dir() -> PathBuf {
        let dir = config_dir().unwrap_or_default()
            .join("dlog");
        dir
    }

    pub fn data_dir() -> PathBuf {
        Self::conf_dir().join("data")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: None,
            data_loc: Some(Self::data_dir()),
            format: Some(FormatConfig::Csv),
            record: None,
            user: None,
            item: None,
            fact: None,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RecordConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactConfig {

}
