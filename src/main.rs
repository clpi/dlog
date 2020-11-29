pub mod files;
pub mod config;
pub mod cmd;
pub mod util;


fn main() -> Result<(), pico_args::Error> {
    println!("Hello, world!");
    cmd::Log::run()?;
    Ok(())
}
