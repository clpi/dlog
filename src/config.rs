use crate::util;
use std::{
    fs, io::{self, prelude::*}, path::PathBuf,
        convert::TryInto, collections::HashMap,
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
    // record: Option<RecordConfig>,
    // item: Option<ItemConfig>,
    // fact: Option<FactConfig>,
    prompt_for_value: bool,
    prompt_for_record: bool,
    prompt_for_units: bool,
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
            prompt_for_units: false,
            prompt_for_record: false,
            prompt_for_value: false,
        }
    }
}

impl DConfig {

    pub fn show(&self) -> () {
        let p = toml::to_string_pretty(&self).unwrap();
        println!("{}", p);
    }

    pub fn load() -> Self { Self::default() }
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

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct DConfig {
    name: Option<String>,
    dialect: Option<String>,
    format: Option<FormatConfig>,
    data_loc: PathBuf,
    // start_of_week: Option<chrono::Weekday>,
    // color: Option<bool>,
    // custom_db: Option<String>,
    // data_dir: Option<String>,
    // date_format: Option<String>,
    // default_editor: Option<String>,
    // encryption: bool,
    // synchronization: bool,
    // password: Option<String>,
    // data_loc: Option<PathBuf>,
    // user: Option<UserConfig>,
    // prompt_for_value: bool,
    // prompt_for_record: bool,
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

    pub fn show(&self) {
        println!("{}", toml::to_string_pretty(self).unwrap())
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

    pub fn path() -> PathBuf {

        fn create_conf(conf_dir: PathBuf) -> PathBuf {
            if conf_dir.exists() {
                let conf = conf_dir.join("config.toml");
                return conf
            } else {
                fs::create_dir(&conf_dir).unwrap_or_default();
                let conf = conf_dir.join("config.toml");
                return conf
            }
        }

        if let Some(conf_dir) = dirs_next::config_dir() {
            let conf_dir = conf_dir.join("dlog");
            create_conf(conf_dir)
        } else if let Some(home_dir) = dirs_next::home_dir() {
            let conf_dir = home_dir.join(".dlog");
            create_conf(conf_dir)
        } else {
            let conf_dir = dirs_next::data_dir().expect("No valid dir");
            create_conf(conf_dir)
        }
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


*/


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

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordConfig {
    init_behavior: InitBehavior
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

#[derive(Debug, Serialize, Deserialize)]
pub enum InitBehavior {
    CurrentDir,
    DataDir,
    Documents,
    Home,
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
