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

    pub fn print_help() {
        println!("Fact help")
    }
}

impl<'a> From<&'a ArgMatches<'a>> for Fact {
    fn from(matches: &ArgMatches<'a>) -> Self {
        if matches.args.len() == 0 { Self::print_help() }
        println!("Fact");
        Self::default()
    }
}
