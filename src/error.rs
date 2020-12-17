use std::{fmt, env, io, num, error};
use crate::models::{Item, Record, Fact};

pub type DResult<T> = Result<T, DError>;

#[derive(Debug)]
pub enum DError {
    Io(io::Error),
    Csv(csv::Error),
    Config(toml::de::Error),
    NotFound,
}

#[derive(Debug)]
pub enum NotFoundError {
    ItemNotFound,
    FactNot
}

impl error::Error for DError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            DError::Io(ref err) => Some(err),
            DError::Csv(ref err) => Some(err),
            DError::Config(ref err) => Some(err),
            DError::NotFound => None,
        }
    }

}

impl fmt::Display for DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DError::Io(ref err) => write!(f, "IO error: {}", err),
            DError::Config(ref err) => write!(f, "Config TOML parse err {}", err),
            DError::Csv(ref err) => write!(f, "Csv error: {}", err),
            DError::NotFound => write!(f, "What you were looking for is not there"),
        }
    }
}

impl From<io::Error> for DError {
    fn from(err: io::Error) -> Self {
        DError::Io(err)
    }
}

impl From<csv::Error> for DError {
    fn from(err: csv::Error) -> Self {
        DError::Csv(err)
    }
}

