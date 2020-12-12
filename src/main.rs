pub mod cmd;
pub mod config;
pub mod util;
pub mod csv;

use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
