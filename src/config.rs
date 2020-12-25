use crate::util;
use std::{
    fs, io::{self, prelude::*}, path::PathBuf,
        convert::TryInto, collections::HashMap,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FormatConfig {
    Csv,
    Json,
    Yaml,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DConfig {
    name: Option<String>,
    dialect: Option<String>,
    start_of_week: Option<chrono::Weekday>,
    color: Option<bool>,
    custom_db: Option<String>,
    data_dir: Option<String>,
    date_format: Option<String>,
    default_editor: Option<String>,
    encryption: bool,
    synchronization: bool,
    password: Option<String>,
    format: Option<FormatConfig>,
    data_loc: Option<PathBuf>,
    record: Option<RecordConfig>,
    item: Option<ItemConfig>,
    fact: Option<FactConfig>,
    user: Option<UserConfig>,
}

impl DConfig {

    pub fn new() -> Self {
        let path = Self::conf_dir().join("config");
        let mut cf = config::Config::default();
        cf.merge(config::File::with_name(path.to_str().unwrap()))
            .unwrap()
            .merge(config::Environment::with_prefix("APP")).unwrap();
        println!("{:?}", cf.try_into::<HashMap<String, String>>().unwrap());
        Self::default()
    }

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

    pub fn default_config() -> DConfig {
    let toml_str = r#"
        [format]
        [user]
        [record]
        [item]
        [fact]
    "#;
    let conf: DConfig = toml::from_str(toml_str).unwrap();
    conf
    }
}

impl Default for DConfig {
    fn default() -> Self {
        Self {
            name: None,
            data_loc: Some(Self::data_dir()),
            format: Some(FormatConfig::Csv),
            record: None,
            user: None,
            item: None,
            fact: None,
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
