use dlog_lib::cmd::{DApp, Cmd};
use std::io;

fn main() -> io::Result<()> {
    DApp::default().run();
    Ok(())
}
