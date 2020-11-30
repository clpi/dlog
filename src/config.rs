use dirs_next::{config_dir, data_local_dir};
use std::{
    io, fs, path::{Path, PathBuf},
};
use serde::{Serialize, Deserialize};
use toml::de;

const CONFIG_DIR: &'static str = "dlog";
const CONFIG_PATH: &'static str = "dlog.toml";
const DATA_PATH: &'static str = "dlog";
const DEFAULT_CONF: &'static str = "assets/Config.default.toml";

pub struct ConfigPath {
    _conf: &'static str,
    _data: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    path: String,
    user: String,
    data_dir: String,
}

impl Config {


    pub fn load(path: String) -> io::Result<Self> {
        let _conf = fs::read_to_string(&path)?;

        Ok(Self { path, ..Self::default() })
    }
}

impl Default for Config {
    fn default() -> Self {
        let _conf = fs::read_to_string("~/.config/dlog/");
        Self { user: String::new(), ..Default::default() }
    }
}

impl ConfigPath {

    /// If the user does not have ~/.config/dlog/, create it
    pub fn new() -> io::Result<Self> {
        return Ok(Self::default())
    }

    pub fn check_default_config_exists() -> Option<PathBuf> {
        let conf = config_dir().expect("Could not get config dir")
            .join("dconf");
        if conf.exists() && conf.is_dir() {
            Some(conf)
        } else {
            None
        }
    }

    // pub fn check_default_data_exists() -> Option<PathBuf> {

    // }

    pub fn init() -> io::Result<()> {
        let _conf_file = "dlog.toml";
        let _dir_name = "dlog";
        Ok(())
    }

    pub fn read_default_conf(self) -> io::Result<()> {
        Ok(())
    }

    pub fn create_default_conf(self, custom_path: Option<String>) -> io::Result<()> {
        if let Some(path) = custom_path {
            let path = std::path::Path::new(path.as_str());
            if !path.parent().unwrap().is_dir()
            || !path.parent().unwrap().exists() {
                fs::create_dir_all(path.parent().unwrap())?;
            }
            if !path.exists() || path.is_file() {
                let _conf = fs::File::create(path)?;
            }
        }
        let _config = if let Some(config_dir) = config_dir() {
            let dlog_conf_dir = config_dir.join(CONFIG_DIR);
            if !dlog_conf_dir.exists() || !dlog_conf_dir.is_dir() {
                let conf_path = dlog_conf_dir.join(CONFIG_PATH);
                let _conf_file = fs::File::create(conf_path)?;
                let mut _default = String::new();
                fs::read_to_string(DEFAULT_CONF)?;
                let _conf_toml: Config = toml::from_str(_default.as_str())?;
            }
        };
        Ok(())
    }

    pub fn create_data_dir() -> io::Result<()> {
        Ok(())
    }
}

impl Default for ConfigPath {
    fn default() -> Self {
        Self {
            _conf: "~/.config/dlog/dlog.toml",
            _data: "~/.dlog"
        }
    }
}
