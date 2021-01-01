use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Notes {
    pub notes: String,
}

impl Notes {
    pub fn new(note: &str) -> Self {
        Self { notes: note.to_string() }
    }
}

impl fmt::Display for Notes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.notes.as_str())
    }
}
