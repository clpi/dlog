use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Item;

impl Item{

    pub fn new() -> Self {
        Self::default()
    }

    pub fn cmd() -> clap::App<'static> {
        clap::App::new("item")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])

    }

    pub fn print_help() {
        println!("Item help")
    }
}

impl From<&ArgMatches> for Item {
    fn from(matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Item");
        Self::new()
    }
}
