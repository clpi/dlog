use chrono::{DateTime, Local};
use crate::{
    args::search::{Search, Filters},
    models::{
        fact::{Fact, AbstractFact},
        units::Units,
        note::Notes,
        item::Item,
        attrib::Attrib,
        record::Record,
    },
    cmd::Cmd,
    prompt::prompt,
};
use clap::{ArgMatches, FromArgMatches, Subcommand};
use colored::{Color, Colorize};

#[derive(Debug)]
pub enum FactCmd {
    New(Fact, AbstractFact),
    NewFactType(AbstractFact),
    Delete(Fact),
    DeleteFactType(AbstractFact),
    Link(Fact),
    LinkFactType(AbstractFact),
    Search(Search),
    Help,
    List,
    Invalid,
}


impl Default for FactCmd {
    fn default() -> Self {
        FactCmd::Help
    }
}

impl Cmd for FactCmd {

    fn run(&self) {
        println!("{}", format!("Running fact cmd...")
            .color(Color::BrightCyan))
    }

    fn name() -> &'static str { "fact" }
    fn about() -> &'static str { "The fact cmd" }
    fn long_about() -> &'static str { "The fact cmd" }
    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            Self::key_arg(1),
            Self::val_arg(2),
            Self::val_unit(3),
            Self::value(),
            Self::units(),
            Self::persist_units(),
            Self::notes(),
            Self::persist_notes(),
            Self::attributes(),
            Self::persist_attributes(),
            Self::record(),
            Self::persist_record(),
            Self::item(),
            Self::persist_item(),
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            Self::search_cmd(),
            Self::list_cmd(),
            Self::help_cmd(),
            Self::delete_cmd(),
            clap::App::new("get")
                .about("Get info about a specific fact")
                .long_flag("get")
                .short_flag('g'),
            clap::App::new("link")
                .about("Link two facts together, or with a record/fact")
                .long_flag("link")
                .short_flag('k')
        ]
    }

    fn print_help() {
        let help = format!("
            FACT: A fact is at its most basic, a key-value pa-\n
                  ir which defines a single piece of info blah\n
                  blah write later                            \n
        ").color(Color::BrightCyan);

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

impl FromArgMatches for FactCmd {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        println!("{}", format!("subc: {:?} \n matches: {:?}",
            matches.subcommand(),
            matches
        ).color(Color::BrightCyan));
        let subc = FactCmd::from_subcommand(matches.subcommand());
        match matches.subcommand() {
            Some(("new", sub)) => {
                let fact = Fact::from_arg_matches(sub);
                let af = AbstractFact::from_arg_matches(sub);
                return FactCmd::New(fact, af)
            },
            Some(("search", sub)) => {
                if let Some(r_filts) = sub.values_of("filterrecord") {
                    println!("{}", "Filter fact in records:");
                    let r_filts = r_filts
                        .inspect(|r| {
                            println!("{}", format!("R: {}", r)
                                .color(Color::BrightCyan));
                        })
                        .collect::<Vec<&str>>();
                }
                if let Some(i_filts) = sub.values_of("filteritem") {
                    println!("{}", "Filter fact in items:");
                    let i_filts = i_filts
                        .inspect(|r| {
                            println!("{}", format!("R: {}", r)
                                .color(Color::BrightCyan));
                        })
                        .collect::<Vec<&str>>();
                }
                let filters = Filters::from_arg_matches(matches);
                return FactCmd::Search(Search::default())
            },
            Some(("list", sub)) => {
                println!("List facts comand, {}", "list");
                return FactCmd::List
            },
            Some(("info", sub)) => {
                println!("Info facts comand");
                return FactCmd::Help
            },
            Some((&_, &_)) => {
                println!("Some other fact cmd");
                return FactCmd::Help
            },
            None => {
                let fact = Fact::from_arg_matches(&matches);
                let fact_type = AbstractFact::from_arg_matches(&matches);
                println!("Got fact: {:?}", fact);
                println!("Got abstract fact: {:#?}", fact_type);
                FactCmd::New(fact, fact_type)
            }
        }
    }
}

impl FactCmd {

    pub fn fact_args() -> Vec<clap::Arg<'static>> {
        vec![
            Self::key_arg(1),
            Self::val_arg(2),
            Self::val_unit(3),
            Self::value(),
            Self::units(),
            Self::persist_units(),
            Self::notes(),
            Self::persist_notes(),
            Self::attributes(),
            Self::persist_attributes(),
            Self::record(),
            Self::persist_record(),
            Self::item(),
            Self::persist_item(),
        ]
    }

    fn search_cmd() -> clap::App<'static> {
        clap::App::new("search")
            .about("Search for a fact")
            .long_flag("search")
            .short_flag('s')
            .args(&[
                clap::Arg::new("attrib")
                    .about("Filter by attribute")
                    .short('a')
                    .long("attrib")
                    .multiple(true)
                    .required(false),
                clap::Arg::new("sort")
                    .about("Sort output results by specified parameter")
                    .possible_values(&[
                        "alphabetical", "entry-quantity", "date", "item",
                        "record", "attrib",
                    ])
                    .case_insensitive(true)
                    .value_name("SORT")
                    .required(false)
                    .takes_value(true),
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
                clap::Arg::new("filteritem")
                    .about("Filter by items")
                    .multiple(true)
                    .long("item")
                    .short('i')
                    .required(false),
                clap::Arg::new("filterrecord")
                    .about("Filter by record(s)")
                    .multiple(true)
                    .long("record")
                    .short('s')
                    .required(false),
                clap::Arg::new("max-results")
                    .about("Maximum number of entries to display")
                    .long("max")
                    .short('m')
                    .takes_value(false)
                    .default_value("50")
                    .value_name("rescount")
                    .required(false),
                clap::Arg::new("case-insensitive")
                    .about("Search for fact case insensitive")
                    .required(false),
            ])
    }

    fn list_cmd() -> clap::App<'static> {
        clap::App::new("list")
            .about("List all of the facts globaally/in record/item")
            .long_about("Specify arguments to list different facts")
            .long_flag("ls")
            .short_flag('l')
            .args(&[
                clap::Arg::new("record")
                    .about("Fact in record")
                    .short('r'),
                clap::Arg::new("item")
                    .about("Fact in item")
                    .short('i'),
                clap::Arg::new("attribute")
                    .about("Fact with attribute")
                    .short('a'),
            ])
    }

    pub fn delete_cmd() -> clap::App<'static> {
        clap::App::new("delete")
            .about("Delete a fact or fact entry")
            .long_about("Specify arguments to list different facts")
            .long_flag("delete")
            .short_flag('d')
            .args(&[
                clap::Arg::new("record")
                    .about("Fact in record")
                    .short('r'),
                clap::Arg::new("item")
                    .about("Fact in item")
                    .short('i'),
                clap::Arg::new("attribute")
                    .about("Fact with attribute")
                    .short('a'),
            ])
    }

    pub fn key_arg(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("NAME")
            .about("Name of the fact to get or make")
            .required(false)
            .validator(|a| crate::prompt::validate_input(a.into()))
            .index(idx)
    }

    pub fn val_arg(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("VALUE") //TODO if no index 3, prompt from stdin
            .requires("NAME")
            .about("Value of the fact given by NAME")
            .validator(|a| crate::prompt::validate_input(a.into()))
            .index(idx)
    }

    pub fn val_unit(idx: u64) -> clap::Arg<'static> {
        clap::Arg::new("UNIT") //TODO if no index 3, prompt from stdin
            .about("First unit value")
            .long_about("Units for the value provided for the input fact. If not provided, defaults to the last units provided or the units specified as the permanent units for this fact.")
            .use_delimiter(true)
            .value_delimiter(" ")
            .require_delimiter(true)
            .requires_all(&["VALUE", "NAME"])
            .overrides_with_all(&["unit", "link-units"])
            .required(false)
            .validator(|a| crate::prompt::validate_input(a.into()))
            .index(idx)
            .multiple(true)
    }

    pub fn value() -> clap::Arg<'static> {
        clap::Arg::new("val") //TODO if no index 3, prompt from stdin
            .about("Set a value for this fact entry")
            .long_about("Set the value of this fact. Can be specified multiple times 'fact -v val1 -v val2'.")
            .requires("NAME")
            .overrides_with("VALUE")
            .multiple_occurrences(true)
            .long("value")
            .alias("val")
            .short('v')
            .required(false)
            .validator(|a| crate::prompt::validate_input(a.into()))
    }

    pub fn units() -> clap::Arg<'static> {
        clap::Arg::new("unit") //TODO if no index 3, prompt from stdin
            .about("Set units for all entries of this fact to the kind implied by the input")
            .long_about("Sets the units for the fact to the type implied by the input string, if applicable (time, length, duration, etc.). If it doesn't conform to a known unit kind, then sets this specific fact entry to have units specified. Future fact entries for this fact, in the absence of explicit input, will infer the units as this unit entry.")
            .short('u')
            .long("units")
            .overrides_with("UNIT")
            .requires_all(&["VALUE", "NAME"])
            .value_name("UNITS")
            .validator(|a| crate::prompt::validate_input(a.into()))
            .multiple_occurrences(true)
            .required(false)

    }

    /// Sets the units for all entries of this fact. If units are of a known type (time, duration
    /// ,etc.) all units of that unit type are valid and the unit type is what is linked. If the
    /// unit does not conform to a known unit type, then adds the provided unit as a known unit for
    /// this fact,
    pub fn persist_units() -> clap::Arg<'static> {
        clap::Arg::new("link-unit") //TODO if no index 3, prompt from stdin
            .about("Set units for all entries of this fact")
            .long_about("Set the units for all entries of this fact, past or future. If this fact already has a unit type linked to it, set to new linked unit value. Unit type (time, length, volume, etc.) will be inferred from the specific units provided, and any units of that unit kind will be valid (i.e. 'minutes' will set unit type to 'time', meaning 'minutes', 'min', 'hours', etc. are valid)")
            .long_about("Save the defined units as the permanent units for this fact")
            .aliases(&["set-unit", "save-unit"])
            .short('U')
            .long("link-units")
            .requires("NAME")
            .overrides_with_all(&["unit", "UNITS"])
            .multiple_occurrences(true)
            .value_name("UNITS")
            .required(false)
    }

    pub fn notes() -> clap::Arg<'static> {
        clap::Arg::new("notes")
            .about("Provide any notes about this specific fact entry")
            .long_about("Add any arbitrary notes to this fact entry, but not to all of this fact's past or future entries")
            .short('n')
            .long("notes")
            .required(false)
            .multiple_occurrences(true)
            .value_name("NOTE")
            .takes_value(true)
            .requires_all(&["VALUE", "NAME"])
    }

    pub fn persist_notes() -> clap::Arg<'static> {
        clap::Arg::new("link-notes")
            .about("Annotate all entries of this fact with the given note string")
            .hidden_short_help(true)
            .long_about("Any extra notes about the fact entry written freeform.")
            .short('N')
            .long("link-notes")
            .multiple_occurrences(true)
            .required(false)
            .takes_value(true)
            .requires("NAME")
            .value_name("NOTE")
    }

    pub fn attributes() -> clap::Arg<'static> {
        clap::Arg::new("attribs")
            .about("Add any attribs desired to the new fact")
            .long_about("Give this fact entry an attribute (but do not add this attribute to all fact entries of this fact from the past or future). If any links are specified")
            .long("attrib")
            .short('a')
            .required(false)
            .validator(|a| crate::prompt::validate_input(a.into()))
            .requires_all(&["VALUE", "NAME"])
            .multiple_occurrences(true)
            .takes_value(true)
            .value_name("ATTRIBUTE")
    }

    pub fn persist_attributes() -> clap::Arg<'static> {
        clap::Arg::new("link-attrib")
            .about("Whether to persist the attribute-fact link")
            .long_about("Link an attribute to this fact (not just this fact entry)")
            .long("link-attrib")
            .aliases(&["save-attrib",  "attrib-link"])
            .short('A')
            .requires("NAME")
            .required(false)
            .multiple_occurrences(true)
            .value_name("ATTRIBUTE")
    }

    pub fn record() -> clap::Arg<'static> {
        clap::Arg::new("record")
            .about("Specify the record to add this fact to")
            .long_about("Link this specific fact entry to a given record (but do not add this fact to the record)")
            .long("record")
            .short('r')
            .required(false)
            .multiple_occurrences(true) // NOTE To specify multiple records, multiple occurrences with at least one val
            .multiple_values(true) // NOTE first value specified is the record name, all past are link attr values
            .settings(&[
                clap::ArgSettings::UseValueDelimiter,
                clap::ArgSettings::MultipleValues
            ])
            .requires_all(&["VALUE", "NAME"])
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["RECORD", "RELATION"]) // FIXME want to accept multiple relations,
                                                  // FIXME want to accept key,value relations.
    }

    pub fn persist_record() -> clap::Arg<'static> {
        clap::Arg::new("link-record")
            .about("Whether to persist the record-fact link specified")
            .long_about("Link a record to this fact (not just this fact entry)")
            .long("link-record")
            .aliases(&["save-record", "save-rec", "record-link"])
            .short('R')
            .multiple_occurrences(true) // NOTE To specify multiple records, multiple occurrences with at least one val
            .multiple_values(true) // NOTE first value specified is the record name, all past are link attr values
            .requires("NAME")
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["RECORD", "RELATION"])
    }

    pub fn item() -> clap::Arg<'static> {
        clap::Arg::new("item")
            .about("Specify the item to add this fact to")
            .long_about("Link this specific fact entry to a given item (but do not add this fact to the item)")
            .long("item")
            .overrides_with("link-item")
            .short('i')
            .required(false)
            .requires_all(&["VALUE", "NAME"])
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["ITEM", "RELATION"])
    }

    pub fn persist_item() -> clap::Arg<'static> {
        clap::Arg::new("link-item")
            .about("Whether to persist the item-fact link specified")
            .long_about("Link an item to this fact (not just this fact entry)")
            .long("link-item")
            .aliases(&["save-item",  "item-link"])
            .short('I')
            .overrides_with("item") //TODO test this
            .requires("NAME")
            .required(false)
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["ITEM", "RELATION"])
    }

    pub fn fact() -> clap::Arg<'static> {
        clap::Arg::new("fact")
            .hidden_short_help(true)
            .long_about("Specify a fact type or fact entry (dependent on the value provided) to link to this fact entry. Ex. For the cmd 'sleep 5 hr -f 'alcohol 3 shots', a new fact entry 'alcohol 3 shots' will be created and linked to this fact entry. The cmd 'sleep 5 hr -f 'alcohol' will link the fact type 'alcohol' to this fact entry. ")
            .long_about("Link this specific fact entry to a given item (but do not add this fact to the item)")
            .long("fact")
            .short('f')
            .required(false)
            .requires_all(&["VALUE", "NAME"])
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["FACT", "RELATION"])
    } //NOTE can specify links to facts for a fact entry and also link facts in ths same cmd

    pub fn persist_fact() -> clap::Arg<'static> {
        clap::Arg::new("link-fact")
            .hidden_short_help(true)
            .long_about("Specify a fact type to link to this fact. Ex. For the cmd 'sleep -f alcohol', the existing or new fact 'alcohol' will be linked to this fact entry. The cmd 'sleep 5 hr -f 'alcohol' will link the fact type 'alcohol' to this fact.")
            .long_about("Link this specific fact entry to a given item (but do not add this fact to the item)")
            .long("item")
            .short('i')
            .required(false)
            .requires("NAME")
            .min_values(1)
            .multiple_occurrences(true)
            .multiple_values(true)
            .value_names(&["FACT", "RELATION"])
    }

}


impl clap::Subcommand for FactCmd {
    fn from_subcommand(subcommand: Option<(&str, &ArgMatches)>) -> Option<Self> {
        if let Some((subc, m)) = subcommand {
            let cmd = match subc {
                "new" => {
                    let fact = Fact::from_arg_matches(m);
                    let af = AbstractFact::from_arg_matches(m);
                    Some(Self::New(fact, af))
                },
                "list" => Some(Self::List),
                "search" => Some(Self::Search(Search::from_arg_matches(m))),
                "help" => Some(Self::Help),
                _ => None,
            };
            cmd
        } else {
            None
        }
    }

    fn augment_subcommands(app: clap::App<'_>) -> clap::App<'_> {
        app
    }
}


