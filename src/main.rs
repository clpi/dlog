pub mod cmd;
pub mod models;
pub mod csv;
pub mod config;
pub mod util;
pub mod error;

pub use error::DResult;

use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
