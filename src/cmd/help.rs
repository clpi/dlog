use colored::Colorize;
use pico_args::Arguments;
pub struct HelpCmd {}

impl HelpCmd {

    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog help new".bright_purple());
        println!("{}", format!("H: {:#?}", name).bright_purple());
        println!("{:#?}", args);
        Ok(())
    }
    pub fn parse(args: &Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog help".bright_green());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for HelpCmd {
    fn default() -> Self {
        Self {}
    }
}
