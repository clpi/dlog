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
    name: String,
    format: FormatConfig,
    data_loc: PathBuf,
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
            name: String::new(),
            data_loc: Self::data_dir(),
            format: FormatConfig::Csv,
        }
    }
}

