pub mod store;
pub mod cmd;
pub mod models;
pub mod csv;
pub mod config;
pub mod util;
pub mod prompt;

pub use error::DResult;
use std::{io, sync::RwLock};

lazy_static::lazy_static! {
    static ref CONF: RwLock<Vec<u8>> = RwLock::new(Vec::new());
}

