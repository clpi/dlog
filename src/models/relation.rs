use std::boxed::Box;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relation<R, S> {
    pub name: String,
    pub r1: Box<R>,
    pub r2: Box<S>,
    pub val: Option<String>,
}

impl<R, S> Relation<R, S> {

    pub fn new(name: &str, val: Option<String>, r1: R, r2: S) -> Self {
        Self {
            name: name.to_string(), val,
            r1: Box::new(r1),
            r2: Box::new(r2),
        }
    }
}

impl<R, S> fmt::Display for Relation<R, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())?;
        if let Some(val) = &self.clone().val {
            f.write_str(val)?;
        }
        Ok(())
    }
}
