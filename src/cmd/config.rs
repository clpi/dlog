use colored::Colorize;
use pico_args::Arguments;
pub struct ConfigCmd {

}

impl ConfigCmd {

    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog config new".bright_blue());
        println!("{}", format!("Conf: {:#?}", name).bright_blue());
        println!("{:#?}", args);
        Ok(())
    }

    pub fn parse(args: &Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog config".bright_blue());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for ConfigCmd {
    fn default() -> Self {
        Self {}
    }
}
