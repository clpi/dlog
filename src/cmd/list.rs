use colored::Colorize;
use pico_args::Arguments;
pub struct ListCmd {

}

impl ListCmd {

    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog list new".bright_magenta());
        println!("{}", format!("L: {:#?}", name).bright_magenta());
        println!("{:#?}", args);
        Ok(())
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog list".bright_magenta());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for ListCmd {
    fn default() -> Self {
        Self {}
    }
}
