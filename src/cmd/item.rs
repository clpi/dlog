use colored::{Color, Colorize};
use crate::{
    models::Item,
    cmd::Cmd,
};
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug)]
pub enum ItemCmd {
    New(Option<Item>),
    Delete(Option<Item>),
    List,
    Help,
}

impl Default for ItemCmd {
    fn default() -> Self {
        ItemCmd::New(None)
    }
}

impl Cmd for ItemCmd {

    fn cmd() -> clap::App<'static> {
        clap::App::new("item")
            .about("items")
            .subcommands(vec![
                Self::new_cmd(),
                Self::search_cmd(),
                clap::App::new("list")
                    .about("List all of the items globaally or in a record")
                    .long_flag("ls")
                    .short_flag('l'),
                clap::App::new("get")
                    .about("Get info about a specific item")
                    .long_flag("get")
                    .short_flag('g'),
                clap::App::new("link")
                    .about("Link two items together, or with a record/fact")
                    .long_flag("link")
                    .short_flag('k')
            ])
            .args(&vec![
                clap::Arg::new("help")
                    .about("Display help pertaining to items")
                    .short('h')
                    .long("help")
                    .takes_value(false)
                    .exclusive(true),
                Arg::new("NAME")
                    .about("Name of the item to log")
                    .required(false)
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .index(1),
                Arg::new("FACT") //TODO if no index 3, prompt from stdin
                    .about("Optional fact to associate with new item")
                    .required(false)
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .index(2),
                Arg::new("FACTVAL") //TODO if no index 3, prompt from stdin
                    .about("Optional value of the fact to associate with new fact")
                    .required(false)
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .index(3),
                Arg::new("uncategorized")
                    .aliases(&["misc", "uncat", "etc"])
                    .short('u')
                    .long("uncategorized")
                    .about("Whether to show items part of the inbox record")
                    .takes_value(false),
                Arg::new("attribs")
                    .about("Add any attribs desired to the new item")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .multiple(true),
                Arg::new("record")
                    .about("Specify the record to add this fact to")
                    .long("record")
                    .short('r')
                    .required(false)
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .multiple(true),
                Arg::new("link-attribute")
                    .long("Whether to persist the attribute-item link")
                    .long_about("Link an attribute to this item (not just this fact entry)")
                    .long("link-attrib")
                    .aliases(&["save-attrib",  "attrib-link"])
                    .short('A')
                    .overrides_with("attribs") //TODO test this
                    .multiple(true)
                    .required(false),
                Arg::new("link-record")
                    .about("Whether to persist the record-item link specified")
                    .long_about("Link a record to this item (not just this item entry)")
                    .long("link-record")
                    .aliases(&["save-record", "save-rec", "record-link"])
                    .short('R')
                    .overrides_with("record") //TODO test this
                    .multiple(true)
                    .required(false),
            ]) //TODO add item-item possibility
    }

    fn run(&self) {
        println!("{}", format!("Running item cmd...")
            .color(Color::BrightMagenta))

    }

    fn print_help() {
        let help = format!("
            ITEM: Items define groups of related facts into\n
                  logical clusterings which map to real world\n
                  blah blah blah to be added later\n
        ").color(Color::BrightMagenta);
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

impl ItemCmd {
    pub fn new_cmd() -> clap::App<'static> {
        clap::App::new("new")
            .about("Create a new item to associate with different facts")
            .long_flag("new")
            .short_flag('n')
            .aliases(&["add, create"])
            .args(&[
                clap::Arg::new("record")
                    .about("Specifies the record to add this new item to; inbox if none")
                    .aliases(&["r", "rec"])
                    .long("record")
                    .short('r')
                    .required(false)
                    .multiple(true)
                    .takes_value(true),
                clap::Arg::new("attrib")
                    .about("Add any tags desired to the new item")
                    .long("attrib")
                    .short('a')
                    .required(false)
                    .multiple(true),
                clap::Arg::new("NAME")
                    .about("The name of the item to be added")
                    .required(false)
                    .index(1),
            ])
    }

    fn search_cmd() -> clap::App<'static> {
        clap::App::new("search")
            .about("Search for an item")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
                    .multiple(true)
                    .required(false),
                clap::Arg::new("record")
                    .about("Filter by record(s)")
                    .multiple(true)
                    .long("record")
                    .short('s')
                    .multiple(true)
                    .required(false),
                clap::Arg::new("ascending")
                    .about("Sort values ascending")
                    .long("ascending")
                    .alias("asc")
                    .takes_value(false),
                clap::Arg::new("descending")
                    .about("Sort values ascending")
                    .long("descending")
                    .alias("desc")
                    .takes_value(false),
                clap::Arg::new("max-entries")
                    .about("Maximum number of entries to display")
                    .long("max")
                    .short('m')
                    .takes_value(false)
                    .required(false)
            ])
    }
}

impl FromArgMatches for ItemCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for ItemCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            if sub == "item" {
                Some(Self::from_arg_matches(args))
            } else {
                None
            }
        } else { None }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}
