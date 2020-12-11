use clap::ArgMatches;

#[derive(Default, Debug)]
pub struct Fact;

impl Fact{

    pub fn cmd() -> clap::App<'static, 'static> {
        clap::SubCommand::with_name("fact")
            .about("items")
            .subcommands(vec![
                clap::SubCommand::with_name("new")
            ])
            .args(&vec![
                clap::Arg::with_name("help")
            ])

    }
}

impl<'a> From<&'a ArgMatches<'a>> for Fact {
    fn from(matches: &ArgMatches<'a>) -> Self {
        println!("Fact");
        Self::default()
    }
}
