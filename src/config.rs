use dirs_next::{config_dir, data_local_dir};
use std::{
    io, fs, path::Path,
};
use serde::{Serialize, Deserialize};
use toml::de;

const CONFIG_DIR: &'static str = "dlog";
const CONFIG_PATH: &'static str = "dlog.toml";
const DATA_PATH: &'static str = "dlog";

pub struct ConfigPath{
    conf_dir: &'static str,
    conf_file: &'static str,
    data_dir: &'static str,
}

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
        let conf = fs::read_to_string("~/.config/dlog/");
        Self { user: String::new(), ..Default::default() }
    }
}

impl ConfigPath {

    pub fn new() -> io::Result<Self> {
        if !exists(config_dir().unwrap().join("dconf"))
    }

    pub fn check_default_config_exists() -> io::Result<PathBuf> {
        let conf = config_dir().expect("Could not get config dir")
            .join("dconf");
        if conf.exists() && conf.is_dir() {
            conf
        }
    }

    pub fn init() -> io::Result<()> {
        let conf_file = "dlog.toml";
        let dir_name = "dlog";
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
                let conf = fs::File::create(path)?;
            }
        }
        let config = if let Some(config_dir) = config_dir() {
            let dlog_conf_dir = config_dir.join(self.conf_dir);
            if Self::check_default_config_exists()? {
            if !dlog_conf_dir.exists() || !dlog_conf_dir.is_dir() {
                fs::create_dir(dlog_conf_dir.join(self.conf_file))?;
                let mut file = String::new();
                fs::read_to_string(dlog_conf_dir.join(self.conf_file))?;
                let conf_toml: Config = toml::from_str(file.as_str())?;
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
            conf_dir: "dlog",
            conf_file: "dlog.toml",
            data_dir: "dlog",
        }
    }
}
