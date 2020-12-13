pub mod cmd;
pub mod config;
pub mod util;

use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
