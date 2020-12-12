use super::Cmd;
use colored::{Colorize, Color};
use clap::{ArgMatches, FromArgMatches};

#[derive(Debug,)]
pub enum LinkCmd {
    New(Link),
    Help,
    List,
}

impl Default for LinkCmd {
    fn default() -> Self {
        LinkCmd::Help
    }
}

impl Cmd for LinkCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("LinkCmd")
            .about("LinkCmds")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("name")
            ])
    }

    fn run(&self) {
        println!("{}", format!("Running link cmd...")
            .color(Color::BrightYellow))

    }

    fn print_help() {
        let help = format!("
            LinkCmd: The link command allows for two different\n
                  entities to be connected and have a basis\n
                  for which relational attributes can be added\n
        ").color(Color::BrightYellow);
        println!("> {}", help)
    }

}

impl FromArgMatches for LinkCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for LinkCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "link" {
            Some(Self::from_arg_matches(args))
        } else {
            None
        }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}

#[derive(Debug)]
pub struct Link {

}
