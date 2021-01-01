use serde::{Serialize, Deserialize};
use std::{fmt, collections::HashMap};
use chrono::{DateTime, Local};


///factval
#[derive(Debug, Clone)]
pub struct Value {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FactValue {
    Integer(i32),
    RealNumber(f32),
    Option(HashMap<String, bool>), //TODO find a way to parse this
    ExactTime(DateTime<Local>),
    Duration(std::time::Duration),
    Day(DateTime<Local>),
    Boolean(bool),
    Text(String),
    Range(f32, f32),
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
