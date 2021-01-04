use crate::{
    models::{
        Entry, record::Record
    }, cmd::Cmd,
};
use clap::{ArgMatches, FromArgMatches, Subcommand};

#[derive(Debug)]
pub enum RecordCmd {
    New(Option<Record>),
    List
}

impl Default for RecordCmd {
    fn default() -> Self {
        RecordCmd::New(None)
    }
}

impl Cmd for RecordCmd {

    fn name() -> &'static str { "user" }
    fn about() -> &'static str { "The user cmd" }
    fn long_about() -> &'static str { "The user cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("NAME")
                .about("Name of the item to log")
                .required(false)
                .validator(|a| crate::prompt::validate_input(a.into()))
                .index(1),
            clap::Arg::new("link-attribute")
                .long("Link an attribute to this record")
                .long_about("Link an attribute to this fact (not just this fact entry)")
                .long("link-attrib")
                .aliases(&["save-attrib",  "attrib-link"])
                .short_alias('a')
                .short('A')
                .overrides_with("attribs") //TODO test this
                .requires("NAME")
                .multiple(true)
                .required(false),
            clap::Arg::new("link-item")
                .about("Link (add) an item to this record")
                .long_about("Add an item linkage between a provided record name and an item (whose name is fulfilled by this val)")
                .long("link-item")
                .aliases(&["save-item",  "item-link"])
                .short('I')
                .short_alias('i')
                .overrides_with("item") //TODO test this
                .requires("NAME")
                .multiple(true)
                .required(false),
            clap::Arg::new("link-fact")
                .about("Whether to persist the item-fact link specified")
                .long_about("Link an item to this fact (not just this fact entry)")
                .long("link-item")
                .aliases(&["save-item",  "item-link"])
                .short('I')
                .short_alias('i')
                .requires("NAME")
                .overrides_with("item") //TODO test this
                .multiple(true)
                .required(false),
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            Self::new_cmd(),
            Self::search_cmd(),
            Self::help_cmd(),
            clap::App::new("list")
                .about("List all records")
                .long_flag("ls")
                .short_flag('l'),
            clap::App::new("get")
                .about("Get info about a specific record")
                .long_flag("get")
                .short_flag('g'),
            clap::App::new("link")
                .about("Link two records together, or with a item/fact")
                .long_flag("link")
                .short_flag('k')
        ]
    }


    fn run(&self) {
        println!("{}", format!("Running record cmd...")
            .color(Color::BrightGreen))
    }

    fn print_help() {
        let help = format!("
            RECORD: Define a grouping of items to keep track \n
                    of in different records, and assign diff-\n
                    erent actions and attributes based on... \n
        ").color(Color::BrightGreen);
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

impl FromArgMatches for RecordCmd {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match matches.value_of("NAME") {
            Some(name) => {
                println!("Got new record: {}", &name);
                Self::default()
            },
            None => {
                println!("Received no fact name, provide: to inbox");
                Self::default()
            }
        }
    }
}

// NOTE Don't think this is supposed to be implemented yet
//      until functionality is finished for clap 3.0 but
//      implementing anyways
impl Subcommand for RecordCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((sub, args)) = sub {
            if sub == "record" {
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

impl RecordCmd {
    fn new_cmd() -> clap::App<'static> {
        clap::App::new("new")
            .about("Create a new record")
            .long_flag("new")
            .short_flag('n')
            .aliases(&["create"])
            .args(&[
                clap::Arg::new("record")
                    .about("Specifies the record to add this new item to; inbox if none")
                    .aliases(&["r", "rec"])
                    .long("record")
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .short('r')
                    .required(false)
                    .takes_value(true)
                    .multiple(true),
                clap::Arg::new("attrib")
                    .about("Add any tags desired to the new record")
                    .long("attrib")
                    .short('a')
                    .validator(|a| crate::prompt::validate_input(a.into()))
                    .required(false)
                    .multiple(true),
                clap::Arg::new("NAME")
                    .about("The name of the record to be added")
                    .validator(|a| crate::prompt::validate_input(a.into()))
            ])
    }

    fn search_cmd() -> clap::App<'static> {
        clap::App::new("search")
            .about("Search for a record")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
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
                    .default_value("50")
                    .takes_value(false)
                    .required(false)
            ])
    }
}

