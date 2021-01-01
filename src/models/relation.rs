use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation {
    pub name: String,
    pub val: Option<String>,
}

impl Relation {

    pub fn new(name: &str, val: Option<String>) -> Self {
        Self {
            name: name.to_string(), val
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())?;
        if let Some(val) = self.clone().val {
            f.write_str(val.as_str())?;
        }
        Ok(())
    }
}
