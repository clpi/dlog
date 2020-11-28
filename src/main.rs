pub mod config;
pub mod cmd;


fn main() -> Result<(), pico_args::Error> {
    println!("Hello, world!");
    cmd::LogCmd::parse()?;
    Ok(())
}
