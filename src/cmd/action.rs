use crate::{
    models::{
        action::Action,
    },
    cmd::Cmd
};
use clap::{ArgMatches, FromArgMatches};
use colored::{Color, Colorize};

#[derive(Debug)]
pub enum ActionCmd {
    New(Action),
    Help,
    List,
}

impl Default for ActionCmd {
    fn default() -> Self {
        Self::Help
    }
}

impl Cmd for ActionCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("action")
            .about("actions")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("name")
            ])
    }

    fn run(&self) {
        println!("{}", format!("Running action cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("action").color(Color::BrightRed);
        println!("> {}", help)
    }

}

impl FromArgMatches for ActionCmd {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        println!("{}", format!("subc: {:#?} \n matches: {:#?}",
            matches.subcommand(),
            matches
        ).color(Color::BrightCyan));
        match matches.subcommand() {
            Some(("new", sub)) => {
                println!("New action comand");
            },
            Some(("search", sub)) => {
                println!("Search actions comand");
            },
            Some(("list", sub)) => {
                println!("List actions comand");
            }
            Some(("info", sub)) => {
                println!("Info actions comand");
            }
            Some((&_, &_)) => {},
            None => {}
        }
        Self::print_help();
        Self::default()
    }
}
