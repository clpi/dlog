use dlog::cmd;
use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
