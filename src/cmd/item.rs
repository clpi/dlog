use colored::{Color, Colorize};
use crate::{
    models::{Item, Record, fact::{Fact, AbstractFact}},
    cmd::Cmd,
    args::search::Search,
};
use clap::{Arg, ArgMatches, ArgSettings, FromArgMatches};

#[derive(Debug)]
pub enum ItemCmd {
    New(Item),
    Delete(Item),
    AddFact(Item, Fact),
    AddFactType(AbstractFact),
    EditMetadata(Item),
    List,
    Search(Search),
    Help,
}

impl Default for ItemCmd {
    fn default() -> Self {
        ItemCmd::New(Item::default())
    }
}

impl Cmd for ItemCmd {

    fn name() -> &'static str { "item" }
    fn about() -> &'static str { "The item cmd" }
    fn long_about() -> &'static str { "The item cmd" }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            Self::new_cmd(),
            Self::search_cmd(),
            Self::help_cmd(),
            Self::list_cmd(),
            Self::delete_cmd(),
            clap::App::new("get")
                .about("Get info about a specific item")
                .long_flag("get")
                .short_flag('g'),
            clap::App::new("link")
                .about("Link two items together, or with a record/fact")
                .long_flag("link")
                .short_flag('k')
        ]
    }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
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
            ]
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
        clap::App::new("item_help")
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

    fn delete_cmd() -> clap::App<'static> {
        clap::App::new("delete")
            .about("Delete an item from the database")
            .long_flag("delete")
            .short_flag('d')
    }

    // TODO -- implement this in a trait body and implement the trait
    // for items, records, facts, maybe attribs and relations?
    // Since so many of the args and potential options are shared between all
    fn list_cmd() -> clap::App<'static> {
        clap::App::new("list")
            .about("List all of the items globaally or in a record")
            .long_flag("ls")
            .short_flag('l')
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
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        println!("{}", format!("ITEM: subc: {:#?} \n matches: {:#?}",
            matches.subcommand(),
            matches
        ).color(Color::Magenta));
        if let Some((sub, args)) = matches.subcommand() {
            let cmd = match sub {
                "new" => Self::New(Item::from_arg_matches(args)),
                "add" => Self::AddFact(Item::default(), Fact::from_arg_matches(args)), //TODO handle diff
                "delete" => Self::Delete(Item::from_arg_matches(args)),
                "search" => Self::Search(Search::from_arg_matches(args)),
                "list" => Self::List,
                _ => {
                    let item = Item::from_arg_matches(args);
                    println!("{}", item.table());
                    // println!("{:#?}", item);
                    return Self::New(item);
                },
            };
            return cmd;
        } else {
            Self::New(Item::from_arg_matches(matches))
        }
    }
}

impl clap::Subcommand for ItemCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            let cmd = match sub {
                "new" => Self::New(Item::from_arg_matches(args)),
                "add" => Self::AddFact(Item::default(), Fact::from_arg_matches(args)), //TODO handle diff
                "delete" => Self::Delete(Item::from_arg_matches(args)),
                "search" => Self::Search(Search::from_arg_matches(args)),
                "list" => Self::List,
                _ => Self::New(Item::from_arg_matches(args)),
            };
            Some(cmd)
        } else {
            None
        }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_>
    {
        app
    }
}
