pub mod error;
pub mod files;
pub mod config;
pub mod cmd;
pub mod util;
pub mod types;

fn main() -> Result<(), pico_args::Error> {
    cmd::Log::run()?;
    Ok(())
}
