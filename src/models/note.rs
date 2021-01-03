use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Notes(Vec<Note>);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Note {
    pub notes: String,
}

impl Note {

    pub fn new(note: &str) -> Self {
        Self { notes: note.to_string() }
    }

    pub fn join(notes: &Vec<Self>) -> String {
        notes.iter()
            .map(|a| a.notes.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn prompt(kind: &str, name: &str) -> Result<Note, Box<dyn std::error::Error>> {
        let prompt = format!("Enter any notes for the {}, {}: ", kind, name);
        let notes = crate::prompt::prompt(&prompt)?;
        Ok(Self::from(notes))
    }
}

impl Notes {
    pub fn new(notes: Vec<String>) -> Self {
        let notes: Vec<Note> = notes.iter()
            .map(|n| Note::new(n)).collect();
        Self(notes)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.notes.as_str())
    }
}

impl From<String> for Note {
    fn from(notes: String) -> Self {
        Self { notes }
    }

}

