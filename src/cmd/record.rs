use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Record;

impl Record{

    pub fn cmd() -> clap::App<'static> {
        clap::App::new("record")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])

    }
    pub fn print_help() {
        println!("Record help")
    }
}

impl From<&ArgMatches> for Record {
    fn from(matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Record");
        Self::default()
    }
}
