use colored::Colorize;
use pico_args::Arguments;

pub struct RecordCmd {

}

impl RecordCmd {

    pub fn new(name: String, args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog record automatic new".bright_cyan());
        println!("{}", format!("R: {:#?}", name).bright_cyan());
        println!("{:#?}", args);
        Ok(())
    }

    pub fn parse(args: &mut Arguments) -> Result<(), pico_args::Error> {
        println!("{}", "dlog record".bright_cyan());
        println!("{:#?}", args);
        Ok(())
    }
}

impl Default for RecordCmd {
    fn default() -> Self {
        Self {}
    }
}

impl ToString for RecordCmd {
    fn to_string(&self) -> String {
        "record".to_string()
    }
}
