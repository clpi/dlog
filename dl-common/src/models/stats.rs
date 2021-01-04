use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Stats {
    pub facts: u32,
    pub items: u32,
    pub records: u32,
    pub attribs: u32,
    pub links: u32,
    pub actions: u32,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(format!(
            "Stats: F: {}, I: {}, R: {}, A: {}, L: {}\n",
            self.facts, self.items, self.records, self.attribs, self.links)
            .as_str())
    }
}

