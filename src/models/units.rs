use crate::prompt::prompt;
use std::{fmt, path::PathBuf, convert::TryFrom, time};
use serde::{Serialize, Deserialize};
use chrono::{Utc, DateTime, Duration, format, prelude::*};
use chrono_english::{Dialect, DateError, parse_date_string};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Units {
    #[serde(rename="Date")]
    Datetime(DateTime<Local>),
    #[serde(skip, rename="Duration")]
    Duration(Duration),
    #[serde(rename="Other")]
    Other(String),
    #[serde(rename="None")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserUnit {
    DiscreteNum(i32),
    ContinuousNum(f32),
    Text(String),
    Enumeration {
        name: String,
        vals: std::collections::HashMap<String, String>,
    },
}


impl Default for Units {
    fn default() -> Self {
        Units::None
    }
}

impl Units {

    pub fn prompt(prompt_str: &str) -> Self {
        let unit = prompt(prompt_str)
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

    pub fn from_time_str<'a>(time: Vec<&'a str>) -> Self {
        while let Some(word) = time.iter().next() {
            if word.chars().all(|c| c.is_numeric()) {
                println!("word is qty");
            } else {
                if let Ok(weekday) = word.parse::<chrono::Weekday>() {
                    let _dt = weekday;
                    let today = Local::now().weekday();
                }
                if word.contains("day") {

                } else if word.contains("hr") || word.contains("hour") {

                } else if word == &"s" || word.contains("sec") {

                } else if word == &"min" || word.contains("minute") {

                } else if word.contains("wk") || word.contains("week") {

                } else if word == &"mo" || word.contains("month") {

                } else if word.contains("year") || word.contains("yr") {

                }
            }

        }
        Self::default()
    }
}

impl From<Option<String>> for Units {
    fn from(input: Option<String>) -> Self {
        if let Some(input) = input { //TODO check if datetime
            if let Ok(date) = chrono_english::parse_date_string::<Local>(&input, Local::now(), Dialect::Us){
                return Units::Datetime(date);
            }
            Units::Other(input)
        } else { Units::None }
    }
}

impl std::str::FromStr for Units {
    type Err = crate::error::DError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(date) = chrono_english::parse_date_string::<Local>(s, Local::now(), Dialect::Us){
            return Ok(Units::Datetime(date));
        }
        if let Some(dur) = s.strip_prefix("for") { //TODO split this into match fn
            if dur.split_whitespace().any(|w| {
                w.contains("min") || w.contains("minute")
                    || w.contains("sec") || w.contains("second")
                    || w.contains("hr") || w.contains("hour")
            })
            {
                return Ok(Units::Duration(Duration::seconds(100)));
            } else {
                return Ok(Units::Other(s.to_string()));
            }
        }
        return Ok(Units::Other(s.to_string()))
    }
}

impl From<Vec<String>> for Units {
    fn from(units: Vec<String>) -> Self {
        Self::default() //TODO implement
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
            Units::Duration(duration) => {
                f.write_str(&duration.num_seconds().to_string())?;
            }
            Units::None => {
                String::new();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum DateStr {
    Year(u8),
    Month(u8),
    Week(u32),
    Day(u32),
    Hour(u32),
    Minute(u32),
    Second(u32),
    Ms(u32),
}


