use colored::{Color, Colorize};
use pico_args::Arguments;
use crate::cmd::SubCommand;
use chrono::{DateTime, Utc};

#[derive(Debug,)]
pub struct Field {
    key: String,
    val: String,
    kind: Option<FieldKind>,
    created: DateTime<Utc>,
}

impl Field {
    fn with_kind(self, kind: FieldKind) -> Self {
        Self {
            kind: Some(kind), ..self
        }
    }
}

impl SubCommand for Field {

    fn cmd_string() -> Vec<&'static str> {
        vec!["field", "f"]
    }

    fn new(key: String, val: Option<String>) -> Self {
        Self {
            key,
            val: val.unwrap_or_default(),
            kind: Some(FieldKind::default()),
            created: Utc::now()
        }
    }

    fn insert(&self) -> Result<(), pico_args::Error> {
        Ok(())
    }

    fn color() -> Color { Color::BrightBlue }

}

impl Default for Field {
    fn default() -> Self {
        let key = Self::prompt_key().unwrap();
        let val = Self::new(key.clone(), None).prompt_value().unwrap();
        Self::new(key, Some(val))
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}

#[derive(Debug)]
pub enum FieldKind {
    String,
    Int,
    Float,
    Bool,
    Date,
}

impl Default for FieldKind {
    fn default() -> Self {
        Self::String
    }
}
