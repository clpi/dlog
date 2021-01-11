use chrono::{DateTime, Local};
use clap::IntoApp;
use crate::cmd::Cmd;
use colored::{Color, Colorize};
use crate::models::{
    Entry, Fact, Record, Item, Attrib,
    note::Note, Action, Relation, fact::{FactValue, Unit},
};
use clap::{ FromArgMatches, ArgMatches };

#[derive(Default, Debug)]
pub struct Search {
    query_str: String,
    filters: Filters,
}

impl Search {

}

impl Cmd for Search {

    fn name() -> &'static str { "search" }
    fn about() -> &'static str { "The search cmd" }
    fn long_about() -> &'static str { "The action cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            Self::help_cmd(),
        ]
    }

    fn run(&self) {
        println!("{}", format!("Running search cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("action").color(Color::BrightRed);
        println!("> {}", help)
    }

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("fact_help")
            .about("Prints help command for fact")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
    }

}

impl IntoApp for Search {
    fn into_app<'help>() -> clap::App<'help> {
        clap::App::new("search")
            .about("")
    }
    fn augment_clap(app: clap::App<'_>) -> clap::App<'_> {
       app.subcommand(Self::into_app())
    }
}

impl clap::FromArgMatches for Search {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Self {
        Search { query_str: "".into(), filters: Filters::None}
    }
}

#[derive(Debug)]
pub enum Filters {
    InItems(Vec<Item>),
    InRecord(Vec<Record>),
    WithAttribute(Vec<Attrib>),
    WithUnit(Vec<Unit>),
    NotesContaining(Vec<String>),
    NameContaining(Vec<String>),
    HasValue(Vec<FactValue>),
    CreatedBefore(DateTime<Local>),
    CreatedAfter(DateTime<Local>),
    WithRelation(Vec<String>),
    None,

}

impl FromArgMatches for Filters {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        Self::HasValue(Vec::new())
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::None
    }
}
