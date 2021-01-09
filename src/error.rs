use std::{fmt, io, num, error};

pub type DResult<T> = Result<T, DError>;

#[derive(Debug)]
pub enum DError {
    Io(io::Error),
    Csv(csv::Error),
    Config(toml::de::Error),
    KeyRejected(ring::error::KeyRejected),
    ParsePath,
    ParseDate,
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
            DError::KeyRejected(ref err) => None,
            DError::ParsePath => None,
            DError::NotFound => None,
            DError::ParseDate => None,
        }
    }

}

impl fmt::Display for DError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DError::Io(ref err) => write!(f, "IO error: {}", err),
            DError::Config(ref err) => write!(f, "Config TOML parse err {}", err),
            DError::Csv(ref err) => write!(f, "Csv error: {}", err),
            DError::ParsePath => write!(f, "Invalid path"),
            DError::NotFound => write!(f, "What you were looking for is not there"),
            DError::KeyRejected(ref err) => write!(f, "Invalid key {}", err),
            DError::ParseDate => write!(f, "Could not parse date"),
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


impl From<std::convert::Infallible> for DError {
    fn from(_: std::convert::Infallible) -> Self {
        DError::ParsePath
    }
}

impl From<ring::error::KeyRejected> for DError {
    fn from(key_error: ring::error::KeyRejected) -> Self {
        DError::KeyRejected(key_error)
    }
}
