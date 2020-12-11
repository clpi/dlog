pub mod cmd;
pub mod config;

use std::io;

fn main() -> io::Result<()> {
    cmd::Dlog::run();
    Ok(())
}
