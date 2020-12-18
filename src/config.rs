use crate::util;
use dirs_next::{config_dir, data_dir, home_dir};
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
    #[serde(skip)]
    timezone: Option<chrono::FixedOffset>,
    dialect: Option<String>,
    color: Option<bool>,
    data_dir: Option<String>,
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

    pub fn dialect(self) -> Option<chrono_english::Dialect> {
        if let Some(dialect) = self.dialect {
            match dialect.to_lowercase().as_str() {
                "en_us" => Some(chrono_english::Dialect::Us),
                "en_uk" => Some(chrono_english::Dialect::Uk),
                _ => None,
            }
        } else { None }
    }

    pub fn conf_dir() -> PathBuf {
        util::get_or_create_conf_dir()
            .expect("Could not get or create conf dir")
    }

    pub fn data_dir() -> PathBuf {
        util::get_or_create_data_dir()
            .expect("Could not get or create conf dir")
    }

    pub fn default_config() -> Config {
    let toml_str = r#"
        [format]
        [user]
        [record]
        [item]
        [fact]
    "#;
    let conf: Config = toml::from_str(toml_str).unwrap();
    conf
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
            timezone: None,
            ..Default::default()
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
