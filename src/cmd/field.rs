use colored::Colorize;
use pico_args::Arguments;
pub struct FieldCmd {}

impl FieldCmd {
    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog field new".bright_yellow());
        println!("{}", format!("F: {:#?}", name).bright_yellow());
        println!("{:#?}", args);
        Ok(())
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog field".bright_yellow());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for FieldCmd {
    fn default() -> Self {
        Self {}
    }
}

impl ToString for FieldCmd {
    fn to_string(&self) -> String {
        "item".to_string()
    }
}
