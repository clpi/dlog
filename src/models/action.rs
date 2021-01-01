use chrono::{DateTime, Local};
use std::{fmt, collections::HashMap};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub cmd: Vec<String>,
    pub created_at: DateTime<Local>
}


impl Action {
    pub fn new(name: String, cmd: Vec<String>) -> Action {
        Self {
            name, cmd,
            created_at: Local::now(),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: crate::prompt::prompt("Please provide the action name").unwrap(),
            cmd: Vec::new(),
            created_at: Local::now(),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Action: {}", self.name))
    }
}

