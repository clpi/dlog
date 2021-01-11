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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurring {
    pub date: Datelike,
    pub event: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelativeTo {
    Now(DateTime<Local>),

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDate {
    year: Option<usize>,
    day_of_month: Option<usize>,
    month: Option<chrono::Month>,
}

#[derive(Debug, Clone, Serialize,  Deserialize)]
pub struct Duration {
    pub secs: u32,
    pub date: Datelike,
}

impl Duration {

    pub fn mins(&self) -> i32 {
        chrono::Duration::seconds(self.secs as i64).num_minutes() as i32
    }

    pub fn done_now(secs: u32) -> Self {
        let cdur = chrono::Duration::seconds(secs as i64);
        Self {
            secs,
            date: Datelike::Datetime(
                if let Some(dt) = Local::now().checked_sub_signed(cdur) { dt
                } else { Local::now() })
        }
    }

    pub fn today(secs: u32) -> Self {
        Self {
            secs,
            date: Datelike::Day(Local::today().naive_local()),
        }

    }

    pub fn begun_now(secs: u32) -> Self {
        Self {
            secs,
            date: Datelike::Datetime(Local::now()),
        }
    }

    pub fn all_day(date: chrono::NaiveDate) -> Self {
        Self {
            secs: 60 * 3600,
            date: Datelike::Day(date),
        }
    }
}

use std::cmp::{min, max};
impl std::str::FromStr for Duration {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut date: Option<Datelike> = None;
        let mut dur: Option<chrono::Duration> = None;
        if let Some(idx) = s.find("at") {
            let time = s.chars().skip(idx).collect::<String>();
            if let Ok(t) = time.parse::<chrono::NaiveTime>() {
                if let Some(dt) = Local::today().and_time(t) {
                    return Ok(Self { secs: 0, date: Datelike::Datetime(dt) });
                } else {
                    return Ok(Self { secs: 0, date: Datelike::Datetime(Local::now()) });
                }
            } else if let Ok(t) = parse_date_string(time.as_str(), Local::now(), Dialect::Us) {
                return Ok(Self { secs: 0, date: Datelike::Datetime(Local::now()) });
            } else {
                return Ok(Self { secs: 0, date: Datelike::Datetime(Local::now()) });
            }
        }
        match (s.find("at"), s.find("from"), s.find("for")) {
            (Some(ati), Some(fromi), Some(fori)) => {
                let first = min(min(ati, fromi), fori);
                let last = max(max(ati, fromi), fori);
                let (first, sr1) = s.split_at(first);
                let (sr2, last) = sr1.split_at(last);
                return Ok(Self { secs: 0, date: Datelike::Datetime(Local::now()) });
            },
            _ => {
                Err(String::from("Could not convert"))
            }
        }
    }
}

impl std::str::FromStr for Datelike {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(date) = chrono_english::parse_date_string(s, Local::now(), Dialect::Us) {
            Ok(Self::Datetime(date))
        } else if let Ok(d) = s.parse::<chrono::NaiveDate>() {
            Ok(Self::Day(d))
        } else if let Ok(m) = s.parse::<chrono::Month>() {
            let _words = s.split_whitespace().map(|w| w.to_string()).collect::<Vec<String>>();
            Ok(Self::Month(m, RelativeTo::Now(Local::now())))
        } else if let Ok(w) = s.parse::<chrono::Weekday>() {
            Ok(Self::Weekday(w, RelativeTo::Now(Local::now())))
        } else {
            Err(String::from("Could not get datelike from str"))
        }
    }
}

pub struct Time {
    at: chrono::NaiveTime,
    duration: Option<std::time::Duration>,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cd = chrono::Duration::seconds(self.secs as i64);
        if self.secs < 60 {
            write!(f, "Duration: {} at {}", self.secs, self.date)
        } else if self.secs < 3600 {
            let mins = cd.num_minutes();
            let s = cd.checked_sub(&chrono::Duration::minutes(mins)).unwrap();
            write!(f, "Duration: {}m, {}s at {}", mins, s, self.date)
        } else {
            let hrs = cd.num_hours();
            let mins = cd.checked_sub(&chrono::Duration::hours(hrs)).unwrap();
            let s = cd.checked_sub(&chrono::Duration::minutes(cd.num_hours())).unwrap();
            write!(f, "Duration: {}h {}m, {}s at {}", hrs, mins, s, self.date)
        }
    }
}

impl fmt::Display for Datelike {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Datelike::Day(d) => write!(f, "Day: {}", d.to_string()),
            Datelike::Weekday(w, RelativeTo::Now(r)) => write!(f, "Weekday: {} from {}", w.to_string(), r),
            Datelike::Datetime(d) => write!(f, "Datetime: {}", d),
            Datelike::Month(m, RelativeTo::Now(r)) => write!(f, "Month {}, Relative to {}", m.number_from_month(), r),
            Datelike::Weekday(w, RelativeTo::Now(r)) => write!(f, "Month {}, Relative to {}", w, r),
            Datelike::Year(y) => write!(f, "year: {}", y),
            Datelike::Week(w) => write!(f, "Week: {}", w),
        }
    }
}
