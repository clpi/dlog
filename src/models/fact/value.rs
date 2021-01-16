use comfy_table::{
    Table, ContentArrangement, presets,
    Cell, Attribute, Color as TColor, ToRow,
};
use crate::{
    csv as csv, prompt,
    models::{
        Entry,
        fact::{Unit, UserUnit},
        record::Record,
        item::Item,
        note::{Note, Notes},
        attrib::{Attrib, Attribs},
        date::{Datelike, Duration, RelativeTo, Recurring},
    },
};
use uuid::Uuid;
use std::{convert::TryFrom, fmt, path::PathBuf, collections::HashMap, str::FromStr};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};
use chrono_english::{parse_date_string, Dialect};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactValue {
    Integer(i32),
    RealNumber(f32),
    Option(HashMap<String, bool>), //TODO find a way to parse this
    Datelike(Datelike),
    Recurring(Recurring),
    Boolean(bool),
    Text(String),
    Range(f32, f32),
    Duration(Duration),
    UserValue(String),
    UserEnum(UserEnum),
    Amount(i32, UserObject),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserObject(String);

impl std::cmp::PartialEq for FactValue {
    fn eq(&self, other: &FactValue) -> bool {
        match (self, other) {
            (Self::Text(t1), Self::Text(t2)) => t1 == t2,
            (Self::Boolean(b1), Self::Boolean(b2)) => b1 == b2,
            (Self::Integer(i1), Self::Integer(i2)) => i1 == i2,
            (Self::Duration(d1), Self::Duration(d2)) => d1.secs.eq(&d2.secs),
            (Self::UserEnum(ue1), Self::UserEnum(ue2)) => {
                return false;
            }
            (Self::Datelike(d1), Self::Datelike(d2)) => match (d1, d2) {
                (Datelike::Day(d1), Datelike::Day(d2)) => d1.eq(&d2),
                (Datelike::Week(w1), Datelike::Week(w2)) => w1.eq(w2),
                (Datelike::Weekday(w1, RelativeTo::Now(r1)), Datelike::Weekday(w2, RelativeTo::Now(r2))) => w1.eq(&w2) && r1.eq(&r2),
                (Datelike::Month(m1, RelativeTo::Now(r1)), Datelike::Month(m2, RelativeTo::Now(r2))) => m1.eq(&m2),
                (Datelike::Year(m1), Datelike::Year(m2)) => m1.eq(m2),
                _ => false,
            },
            _ => false,
        }
    }
}

impl std::cmp::Eq for FactValue {
    fn assert_receiver_is_total_eq(&self) {
    }
}

impl std::hash::Hash for FactValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {

    }
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized, {

    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEnum {
    possible_values: HashMap<FactValue, Option<Unit>>,
    choice: Option<usize>,
}

impl std::str::FromStr for FactValue {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(" ") {
            if let Ok(num) = s.parse::<i32>() {
                return Ok(Self::Integer(num))
            } else if let Ok(num) = s.parse::<f32>() {
                return Ok(Self::RealNumber(num))
            } else if let Ok(dur) = s.parse::<humantime::Duration>() {
                return Ok(Self::Duration(Duration::today(dur.as_secs() as u32)));
            } else if let Ok(time) = parse_date_string(s, Local::now(), Dialect::Us) {
                return Ok(Self::Datelike(Datelike::Datetime(time)))
            } else if let Ok(day) = s.parse::<chrono::Weekday>() {
                return Ok(Self::Datelike(Datelike::Weekday(day, RelativeTo::Now(Local::now()))))
            } else if let Ok(month) = s.parse::<chrono::Month>() {
                return Ok(Self::Datelike(Datelike::Month(month, RelativeTo::Now(Local::now()))))
            } else if let Ok(day) = s.parse::<chrono::NaiveDate>() {
                return Ok(Self::Datelike(Datelike::Day(day)))
            } else if let Ok(b) = s.parse::<bool>() {
                return Ok(Self::Boolean(b))
            } else {
                return Ok(Self::Text(s.to_string()))
            }
        } else {
            let s_s = s.split_whitespace().collect::<Vec<&str>>();
            return Ok(Self::Text(s.to_string()))
        }
    }
}
impl FromArgMatches for FactValue {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        if let Ok(value) = matches.value_of_t::<FactValue>("VALUE") {
            value
        } else {
            Self::Boolean(true)
        }
    }
}

impl Default for FactValue {
    fn default() -> Self {
        FactValue::Boolean(true)
    }
}

impl From<String> for FactValue {
    fn from(val: String) -> Self {
        if let Ok(num) = val.parse::<f32>() {
            FactValue::RealNumber(num)
        } else if let Ok(num) = val.parse::<i32>() {
            FactValue::Integer(num)
        } else if val.contains("-") { // | val.contains("to")?
            let pair = val.split("-").take(2).collect::<Vec<&str>>();
            if let (Ok(n1), Ok(n2)) = (pair[0].parse::<f32>(), pair[1].parse::<f32>()) {
                FactValue::Range(n1, n2)
            } else { FactValue::Text(val) }
        } else {
            match val.as_str() {
                "true" | "yes" | "t" | "y" => FactValue::Boolean(true),
                "false" | "no" | "f" | "n" => FactValue::Boolean(false),
                _ => FactValue::Text(val.into()),
            }
        }
    }
}

impl fmt::Display for FactValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactValue::Text(txt) => f.write_fmt(format_args!("Text val: {}", txt)),
            FactValue::Boolean(b) => f.write_fmt(format_args!("Bool value: {}", b)),
            FactValue::RealNumber(r) => f.write_fmt(format_args!("Real num: {}", r)),
            FactValue::Integer(i) => f.write_fmt(format_args!("Integer: {}", i)),
            FactValue::Duration(d) => write!(f, "{}", d),
            FactValue::Datelike(d) => write!(f, "Datelike {}", d),
            FactValue::Recurring(r) => write!(f, "Recurring: {} every {}", r.date, r.event),
            FactValue::Range(n1, n2) => {
                f.write_fmt(format_args!("Range value from {} to {}",n1, n2))
            },
            FactValue::Option(map) => {
                let mut out = String::new();
                for (opt, sel) in map.iter() {
                    if *sel {
                        out.push_str(&format!("Val opt {} was selected", opt));
                    } else {
                        out.push_str(&format!("Val opt {} not selected", opt));
                    }
                }
                f.write_str(out.as_str())
            },
            _ => { f.write_str("Other") }
        }
    }
}
