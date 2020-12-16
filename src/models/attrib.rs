use crate::{
    util::prompt_input,
    cmd::Cmd,
};
use std::fmt;
use serde::{Serialize, Deserialize};
use clap::{Clap, ArgMatches, FromArgMatches};
use colored::{Colorize, Color};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attrib {
    pub name: String,
}

impl Attrib {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
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

    pub fn prompt(prompt: &str) -> Vec<Attrib> {
        let attrib = prompt_input(prompt)
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
    fn from(string: String) -> Self {
        Self { name: string }
    }
}
