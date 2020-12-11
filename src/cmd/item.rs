use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Item;

impl Item{

    pub fn new() -> Self {
        Self::default()
    }

    pub fn cmd() -> clap::App<'static, 'static> {
        clap::SubCommand::with_name("item")
            .about("items")
            .subcommands(vec![
                clap::SubCommand::with_name("new")
            ])
            .args(&vec![
                clap::Arg::with_name("help")
            ])

    }

    pub fn print_help() {
        println!("Item help")
    }
}

impl<'a> From<&'a ArgMatches<'a>> for Item {
    fn from(matches: &ArgMatches<'a>) -> Self {
        if matches.args.len() == 0 { Self::print_help() }
        println!("Item");
        Self::new()
    }
}
