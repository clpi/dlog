use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Fact;

impl Fact{

    pub fn cmd() -> clap::App<'static> {
        clap::App::new("fact")
            .about("items")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("help")
            ])

    }

    pub fn print_help() {
        println!("Fact help")
    }
}

impl From<&ArgMatches> for Fact {
    fn from(matches: &ArgMatches) -> Self {
        Self::print_help();
        println!("Fact");
        Self::default()
    }
}
