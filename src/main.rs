pub mod cmd;
pub mod models;
pub mod csv;
pub mod config;
pub mod util;

use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
