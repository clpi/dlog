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

    fn name() -> &'static str { "link" }
    fn about() -> &'static str { "The link cmd" }
    fn long_about() -> &'static str { "The link cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("NAME")
                .about("Give an optional name to the linkage")
                .index(1)
                .required(false),
            clap::Arg::new("VALUE")
                .about("Give an optional value to the linkage")
                .required(false)
                .index(2),
            clap::Arg::new("item")
                .short('i')
                .long("item")
                .multiple(true)
                .max_values(2),
            clap::Arg::new("record")
                .short('r')
                .long("record")
                .alias("rec")
                .multiple(true)
                .max_values(2),
            clap::Arg::new("fact")
                .short('f')
                .long("fact")
                .multiple(true)
                .max_values(2),
            clap::Arg::new("attribs")
                .about("Specify any attributes to add to the linkage specified")
                .long_about("Add any number of attributes, separated by spaces or commas, to the specified linkage. Can only be used when using the link cmd to create a new link")
                .short('a')
                .visible_short_alias('A')
                .long("attrib")
                .visible_aliases(&["attribute", "attributes", "attribs"])
                .required(false)
                .multiple(true)
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            clap::App::new("new")
                .visible_aliases(&["create", "add"])
                .short_flag('n')
                .long_flag("new"),
            clap::App::new("list")
                .about("List all linkages given a set of criteria"),
            clap::App::new("search")
                .about("Search for linked facts, items, or records given a set or input arguments"),
            clap::App::new("get")
                .about("Get all links from a provided fact, item, or record")
        ]
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

    fn help_cmd() -> clap::App<'static> {
        clap::App::new("fact_help")
            .about("Prints help command for fact")
            .long_flag("help")
            .short_flag('h')
            .long_about("Prints the help information")
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
