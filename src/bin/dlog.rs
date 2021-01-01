use dlog_lib::cmd;
use std::io;

fn main() -> io::Result<()> {
    cmd::App::run();
    Ok(())
}
