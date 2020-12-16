use crate::util::prompt_input;
use std::{fmt, path::PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Units {
    Datetime(DateTime<Utc>),
    Other(String),
    None,
}

impl Default for Units {
    fn default() -> Self {
        Units::None
    }
}

impl Units {

    pub fn prompt(prompt: &str) -> Self {
        let unit = prompt_input(prompt)
            .expect("Could not prompt fact value");
       Self::from_prompt(unit)
    }

    pub fn from_prompt(prompt: String) -> Self {
        if prompt.len() != 0 {
            let split = prompt.split_whitespace();
            if split.clone().count() == 1 {
                Units::Other(prompt)
            } else { //TODO check if datetime
                let unit: String = split.into_iter().collect();
                Units::Other(unit)
            }
        } else { Units::None }
    }
}

impl From<Option<String>> for Units {
    fn from(input: Option<String>) -> Self {
        if let Some(input) = input { //TODO check if datetime
            Units::Other(input)
        } else { Units::None }
    }
}

impl fmt::Display for Units {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Units::Datetime(date) => {
                f.write_str(date.to_rfc2822().as_str())?;
            },
            Units::Other(units) => {
                f.write_str(units.as_str())?;
            },
            Units::None => {
                String::new();
            }
        }
        Ok(())
    }
}
