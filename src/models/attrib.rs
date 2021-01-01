use crate::{
    prompt::prompt,
    cmd::Cmd,
};
use std::fmt;
use serde::{Serialize, Deserialize};
use clap::{Clap, ArgMatches, FromArgMatches};
use colored::{Colorize, Color};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attrib {
    #[serde(rename = "Name")]
    pub name: String,
    pub value: Option<String>,
}

impl Attrib {
    pub fn new(name: &str, val: Option<String>) -> Self {
        Self { name: name.to_string(), value: None}
    }

    pub fn from_prompt(prompt: String) -> Vec<Attrib> {
        if prompt.len() != 0 {
            let attribs: Vec<Attrib> = prompt.split_whitespace()
                .into_iter()
                .map(|a| Attrib::from(a.to_string()))
                .collect();
            attribs
        } else { vec![] }
    }

    pub fn prompt(prompt_str: &str) -> Vec<Attrib> {
        let attrib = prompt(prompt_str)
            .expect("Could not prompt fact value");
        Self::from_prompt(attrib)
    }

    pub fn join(attribs: &Vec<Self>) -> String {
        let attribs = attribs.clone();
        attribs.iter()
            .map(|a| &a.name)
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl From<String> for Attrib {
    fn from(attrib: String) -> Self {
        if attrib.contains("=") {
            let pair = attrib.split("=").collect::<Vec<&str>>();
            Self { name: pair[0].into(), value: Some(pair[1].into()) }
        } else {
            Self { name: attrib, value: None }
        }
    }
}

impl fmt::Display for Attrib {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(val) = self.value.clone() {
            f.write_fmt(format_args!("Attrib: {} with value {} ", self.name, val))
        } else {
            f.write_fmt(format_args!("Attrib: {}", self.name))
        }

    }
}
