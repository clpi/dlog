use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Item;

impl Item{

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
}

impl<'a> From<&'a ArgMatches<'a>> for Item {
    fn from(matches: &ArgMatches<'a>) -> Self {
        println!("Item");
        Self::default()
    }
}
