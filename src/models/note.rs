use std::{fmt, str::FromStr};
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

    pub fn from_match(matches: Option<clap::Values>) -> Vec<Self> {
        if let Some(notes) = matches {
            notes.map(|a| Note::from_str(a).unwrap()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn join(notes: &Vec<Self>) -> String {
        notes.iter()
            .map(|a| a.notes.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn prompt(kind: &str, name: &str) -> Result<Note, Box<dyn std::error::Error>> {
        let prompt = format!("Enter any notes for the {}, {} (Enter if not applicable): ", kind, name);
        let notes = crate::prompt::prompt(&prompt)?;
        Ok(Self::from_str(notes.as_str()).expect(""))
    }

    pub fn get_matches(matches: &clap::ArgMatches) -> Vec<Self> {
        match matches.values_of_t::<Note>("notes") {
            Ok(m) => m,
            Err(_) => Vec::new(),
        }
    }

    pub fn get_links(matches: &clap::ArgMatches) -> Vec<Self> {
        match matches.values_of_t::<Note>("link-notes") {
            Ok(m) => m,
            Err(_) => Vec::new(),
        }
    }

    pub fn from_col(rec: &csv::StringRecord, col: usize) -> Vec<Self> {
            rec.iter().skip(col)
                .map(|a| Note::new(a))
                .collect()
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
        if self.notes.len() == 0 { f.write_str("(no notes)")?; };
        f.write_str(self.notes.as_str())
    }
}

impl std::str::FromStr for Note {
    type Err = std::convert::Infallible;
    fn from_str(notes:  &str) -> Result<Self, Self::Err> {
        Ok(Self { notes: notes.to_string() })
    }

}

