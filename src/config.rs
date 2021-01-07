use crate::util;
use crate::models::Record;
use std::{
    io::{self, prelude::*, Read, Write},
    path::PathBuf,
    convert::TryInto,
    collections::HashMap,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, )]
pub struct DConfig {
    name: Option<String>,
    aliases: Option<Vec<String>>,
    // fact_aliases: Option<Vec<String>>,
    // record_aliases: Option<Vec<String>>,
    data_dir: PathBuf,
    auth: Option<AuthConfig>,
    start_of_week: chrono::Weekday,
    format: FormatConfig,
    records: Option<Vec<Record>>,
    record: Option<RecordConfig>,
    item: Option<ItemConfig>,
    fact: Option<FactConfig>,
    default_editor: Option<String>,
    // prompt_for_value: bool,
    // prompt_for_record: bool,
    // prompt_for_units: bool,
}


impl Default for DConfig {
    fn default() -> Self {
        Self {
            name: None,
            aliases: None,
            data_dir: util::default_data_dir(None).expect("no valid data dir"),
            auth: None,
            format: FormatConfig::default(),
            start_of_week: chrono::Weekday::Sun,
            record: Some(RecordConfig::default()),
            item: None,
            fact: None,
            default_editor: None,
            records: None,
            // prompt_for_units: false,
            // prompt_for_record: false,
            // prompt_for_value: false,
        }
    }
}

impl DConfig {

    pub fn show(&self) -> () {
        let p = toml::to_string_pretty(&self).unwrap();
        println!("{}", p);
    }

    pub fn load() -> crate::DResult<Self> {
        let mut buf = String::new();
        let mut cf = Self::file()?;
        cf.read_to_string(&mut buf)?;
        if buf.len() == 0 {
            let conf = Self::default();
            cf.write_all(toml::to_string_pretty(&conf).expect("Could not create TOML").as_bytes())?;
            Ok(conf)
        } else {
            let conf: Self = toml::from_str(buf.as_str()).expect("Invalid TOML");
            Ok(conf)
        }
    }

    pub fn file() -> crate::DResult<std::fs::File> {
        let path = Self::default_dir()?
            .join("dlog.toml");
        let cf = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&path)?;
        Ok(cf)
    }

    pub fn default_dir() -> crate::DResult<PathBuf> {
        let path = if let Some(dir) = dirs_next::config_dir() {
            dir.join("dlog")
        } else if let Some(dir) = dirs_next::home_dir() {
            dir.join(".dlog")
        } else if let Some(dir) = dirs_next::document_dir() {
            dir.join("dlog")
        } else if let Some(dir) = dirs_next::desktop_dir() {
            dir.join("dlog")
        } else {
            PathBuf::new().join("~/").join(".dlog")
        };
        let mut dir = std::fs::DirBuilder::new();
        dir.recursive(true);
        dir.create(&path)?;
        Ok(path)
    }

    pub fn set_data_dir(mut self, dir: String) -> Self {
        let dir = PathBuf::new().join(dir);
        self.data_dir = dir;
        self
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FormatConfig {
    time_format: String,
    hour_format: HourFormat,
    date_format: String,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            time_format: "HH:MM:SS".into(),
            date_format: "YYYY-MM-DD".into(),
            hour_format: HourFormat::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    password_cmd: Option<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            username: None,
            email: None,
            password_cmd: None,
            password: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, )]
pub struct RecordConfig {
    init_behavior: InitBehavior,
    inbox: InboxConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxConfig {
    name: String,
    path: PathBuf
}

impl Default for InboxConfig {
    fn default() -> Self {
        Self {
            name: "Inbox".into(),
            path: util::default_data_dir(Some("inbox")).unwrap(),
        }
    }
}
impl Default for RecordConfig {
    fn default() -> Self {
        Self {
            init_behavior: InitBehavior::DataDir,
            inbox: InboxConfig::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FactConfig {
    default_empty_behavior: DefaultFactBehavior,
}

/// Default behavior to occur when 'dlog fact' is run without a fact name
#[derive(Debug, Serialize, Deserialize)]
pub enum DefaultFactBehavior {
    ListMostRecentFacts,
    PromptForFact,
    PrintHelp,
    LisFactsWithAttribute(String),
    ListFactsInRecord(String),
    ListFactsInItem(String)
}

impl Default for DefaultFactBehavior {
    fn default() -> Self {
        Self::ListMostRecentFacts
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {

}

#[derive(Debug, Serialize, Deserialize)]
pub enum InitBehavior {
    CurrentDir,
    DataDir,
    Documents,
    HomeDir,
    Desktop,
    DataLocalDir,
    CustomDir(PathBuf),
}

impl Default for InitBehavior {
    fn default() -> Self {
        Self::DataDir
    }
}

impl std::str::FromStr for InitBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ini = match s.to_lowercase().as_str() {
            "cd" | "current_dir" | "cwd" | "pwd" => Self::CurrentDir,
            "data" | "data_dir" => Self::DataDir,
            "docs" | "documents" => Self::Documents,
            "desktop"  => Self::Desktop,
            "data_local" | "data_local_dir" => Self::DataLocalDir,
            "home" | "~" => Self::HomeDir,
            _ => Self::CustomDir(PathBuf::new().join(s)),
        };
        Ok(ini)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HourFormat {
    TwelveHr,
    TwentyFourHr,
}

impl Default for HourFormat {
    fn default() -> Self {
        HourFormat::TwelveHr
    }
}
