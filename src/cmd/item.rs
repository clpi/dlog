use colored::Colorize;
use pico_args::Arguments;
pub struct ItemCmd {}

impl ItemCmd {

    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog item new".bright_red());
        println!("{}", format!("I: {:#?}", name).bright_cyan());
        println!("{:#?}", args);
        Ok(())
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog item".bright_red());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for ItemCmd {
    fn default() -> Self {
        Self {}
    }
}

impl ToString for ItemCmd {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
