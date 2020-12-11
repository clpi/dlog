pub mod cmd;

use std::io;

fn main() -> io::Result<()> {
    cmd::Dlog::run();
    Ok(())
}
