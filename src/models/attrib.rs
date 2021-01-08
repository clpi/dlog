use crate::prompt::prompt;
use std::{fmt, str::FromStr, collections::HashMap};
use serde::{Serialize, Deserialize};

pub type Attribs = Vec<Attrib>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attrib {
    #[serde(rename = "Name")]
    pub name: String,
    pub value: Option<String>,
}

impl Attrib {
    pub fn new(name: &str, val: Option<String>) -> Self {
        Self { name: name.to_string(), value: val}
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

    pub fn from_match(matches: Option<clap::Values>) -> Vec<Self> {
        if let Some(attribs) = matches {
            attribs.map(|a| Attrib::from(a.to_string())).collect()
        } else {
            Vec::new()
        }
    }

    pub fn prompt(prompt_str: &str) -> Vec<Attrib> {
        let attrib = prompt(prompt_str)
            .expect("Could not prompt fact value");
        Self::from_prompt(attrib)
    }

    pub fn join(attribs: &Vec<Self>) -> String {
        attribs.iter()
            .map(|a| a.clone().to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_matches(matches: &clap::ArgMatches) -> Vec<Self> {
        match matches.values_of_t::<Attrib>("attribs") {
            Ok(m) => m,
            Err(_) => Vec::new(),
        }
    }

    pub fn get_links(matches: &clap::ArgMatches) -> Vec<Self> {
        match matches.values_of_t::<Attrib>("link-attrib") {
            Ok(m) => m,
            Err(_) => Vec::new(),
        }
    }

    pub fn from_col(rec: &csv::StringRecord, col: usize) -> Vec<Self> {
            rec.iter().skip(col)
                .map(|a| Attrib::new(a, None))
                .collect()
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
            f.write_fmt(format_args!("(A: {} V: {}) ", self.name, val))
        } else {
            f.write_fmt(format_args!("(A: {})", self.name))
        }

    }
}

impl Into<String> for Attrib {
    fn into(self) -> String {
        if let Some(val) = self.value.clone() {
            format!("(A: {}, V: {})", self.name, val)
        } else {
            format!("A: {}", self.name)
        }
    }
}

impl std::str::FromStr for Attrib {
    type Err = std::convert::Infallible;
    fn from_str(a:  &str) -> Result<Self, Self::Err> {
        Ok(Self { name: a.to_string(), value: None })
    }
}

