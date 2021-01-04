use dlog::cmd::{App, Cmd};
use std::io;

fn main() -> io::Result<()> {
    App::default().run();
    Ok(())
}
