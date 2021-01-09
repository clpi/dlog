use std::{fmt, convert::TryFrom, str::FromStr};
use chrono_english::{Dialect, parse_date_string};
use crate::error::DError;
use serde::{Serialize, Deserialize};
use humantime::parse_duration;
use humantime_serde::Serde;
use chrono::{prelude::*, DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Datelike {
    Datetime(chrono::DateTime<Local>),
    Day(chrono::NaiveDate),
    Weekday(chrono::Weekday, RelativeTo),
    Month(chrono::Month, RelativeTo),
    Week(u8),
    Year(usize),
}

pub struct RelativeTo(DateTime<Local>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDate {
    year: Option<usize>,
    day_of_month: Option<usize>,
    month: Option<chrono::Month>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    #[serde(with = "humantime_serde")]
    duration: chrono::Duration,
    date: Datelike,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTime {
    hr: u8,
    min: u8,
    sec: u8,
    ms: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelativeDateModifier {
    Today(String),
    Yesterday(String),
    Tomorrow(String),
    Last(String),
    Next(String),
    FromNow(String, u8),
    Ago(String, u8)
}
impl TryFrom<&str> for RelativeDateModifier {
    type Error = DError;
    fn try_from(rm: &str) -> Result<Self, Self::Error> {
        let n = rm.matches(std::char::CharTryFromErro)
        if let Some(today) = rm.matches("today") {

        }
    }
}

impl TryFrom<Vec<String>> for RelativeDateModifier {
    type Error = DError;
    fn try_from(rm: Vec<String>) -> Result<Self, Self::Error> {
        let rem = |s: usize, e: usize, r: Vec<String>| r[s..e].join("");
        match rm[0].as_str() {
            "last" | "previous" | "prev" => Ok(Self::Last(rem(1, rm.len(), rm))),
            "this" | "next" | "upcoming" => Ok(Self::Next(1, rm.len(), rem(rm))),
            "today" | "now" => Ok(Self::Today(rem(1, rm.len(), rm))),
            "tomrrow" => Ok(Self::Tomorrow(rem(1, rm.len(), rm))),
            "yesterday" => Ok(Self::Yesterday),
            _ => match rm[rm.len()].as_str() {
                "ago" => if let Ok(n) = rm[rm.len() - 2].parse::<u8>() {
                    Ok(Self::Ago(rm[rm.len() - 1], n))
                } else {
                    Ok(Self::Ago(rm[0..rm.len() - 1], 1))
                },
                "now" => if rm[rm.len() - 1].as_str() == "from" {
                    if let Ok(n) = rm[rm.len() - 3].parse::<u8>() {
                        Ok(Self::FromNow(rm[rm.len() - 2], n))
                    } else {
                        Ok(Self::FromNow(rm[0..rm.len() -2], 1))
                    }
                },
                _ => Err(DError::ParseDate),
            }

        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    time: Option<chrono::NaiveTime>,
    duration: chrono::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Recurring {
    Every(Datelike, Event),
    NTimesEvery(u8, Event),
    EveryWeekday(RecurringWeekday),
    EveryOtherWeekdayEven(RecurringWeekday),
    EveryOtherWeekdayOdd(RecurringWeekday),
    FirstWeekdayOfMonth(RecurringWeekday),
    SecondWeekdayOfMonth(RecurringWeekday),
    ThirdWeekdayOfMonth(RecurringWeekday),
    FourthWeekdayOfMonth(RecurringWeekday),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringWeekday {
    weekday: chrono::Weekday,
    event: Option<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserObject(String);

impl ToString for Datelike {

}

impl std::str::FromStr for Datelike {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(date) = chrono_english::parse_date_string(s, Local::now(), Dialect::Us) {
            Ok(Self::Datetime(date))
        } else {
            let words = s.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>();
            if words.len() > 1 {
                if let Ok(rm) = RelativeDateModifier::try_from(words[0]) {
                    match rm {
                        RelativeDateModifier::Today(rem) => {
                            if let Some(time) = rem.rfind("at") {

                            }
                            Ok(Datelike::Datetime(Local::now()))
                        },
                        RelativeDateModifier::Tomorrow => Ok(Datelike::Day)
                    }
                }
            }
            // TODO implement
            match s {
                "today" => Ok(Datelike::Today),
                "tomorrow" => Ok(RelativeDate::Next(NaiveDate::Day(chrono::Weekday::Sun), None)),
                "yesterday" => Ok(RelativeDate::Last(NaiveDate::Day(chrono::Weekday::Sun), None)),
                _ => {
                    let s_s = s.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
                    let n = NaiveDate::from_str(s_s[1].as_str())?;
                    match s_s[0].as_str() { //TODO make from_str for relative date
                        "next"  => {return Ok(RelativeDate::Next(n, None))},
                        "last"  => {return Ok(RelativeDate::Next(n, None))},
                        // "every"
                        _ => {return Ok(RelativeDate::Today)}
                    }
                }
            }

        }
    }
}

pub struct Time {
    at: chrono::NaiveTime,
    duration: Option<std::time::Duration>,
}

impl std::str::FromStr for NaiveDate {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(wd) = s.parse::<chrono::Weekday>() {
            Ok(NaiveDate::Day(wd))
        } else {
            Ok(NaiveDate::Day(chrono::Weekday::Sun))
        }
    }
}



