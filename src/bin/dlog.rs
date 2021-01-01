use dlog_lib::cmd::{App, Cmd};
use std::io;

fn main() -> io::Result<()> {
    App::default().run();
    Ok(())
}
