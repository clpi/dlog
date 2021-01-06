pub mod file;

// use chrono::{prelude::*, DateTime, Utc, Weekday, Month, Date};
use colored::{Colorize, Color};
use std::{fs, io::{Read, prelude::*, self}, path::PathBuf};

pub fn default_data_dir(child: Option<&str>) -> crate::DResult<PathBuf> {
    let path = dirs_next::data_dir()
        .or(dirs_next::home_dir())
        .or(dirs_next::data_local_dir())
        .or(dirs_next::document_dir())
        .or(dirs_next::home_dir())
        .or(dirs_next::desktop_dir())
        .expect("No valid default data dir")
        .join("dlog");
    let mut dir = fs::DirBuilder::new();
    dir.recursive(true);
    dir.create(&path)?;
    if let Some(child) = child {
        Ok(path.join(child))
    } else {
        Ok(path)
    }
}

pub fn create_dir(parent: PathBuf, name: &str) -> io::Result<PathBuf> {
    let dir = fs::create_dir(parent.join(name).as_path())?;
    Ok(parent.join(name))
}

pub fn create_default_conf() -> io::Result<()> {
    fs::copy("../assets/Config.default.toml", crate::config::DConfig::conf_dir())?;
    Ok(())
}

pub fn tokenize(input: String) -> Vec<String> {
    let tokens: Vec<String> = input.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    tokens
}

pub fn write_file(input: String, path: PathBuf) -> io::Result<fs::File> {
    use io::Write;
    let mut file = fs::File::create(path.as_path())?;
    write!(file, "{}", input)?;
    Ok(file)
}

pub fn get_or_create_conf_dir() -> io::Result<PathBuf> {
    let def_conf_dir = dirs_next::config_dir()
        .expect("Couldn't find default config dir")
        .join("dlog");
    if !def_conf_dir.exists() || !def_conf_dir.is_dir() {
        fs::create_dir(&def_conf_dir)?;
    }
    Ok(def_conf_dir)
}

pub fn get_or_create_data_dir() -> io::Result<PathBuf> {
    let def_data_dir = dirs_next::data_dir()
        .expect("Could not find default data dir")
        .join("dlog");
    if !def_data_dir.exists() || !def_data_dir.is_dir() {
        fs::create_dir(&def_data_dir)?;
    }
    Ok(def_data_dir)
}


pub fn split_datetime(datetime: chrono::DateTime<chrono::Utc>) -> () {

}
