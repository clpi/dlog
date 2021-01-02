use crate::{
    models::attrib::Attrib,
    cmd::Cmd,
};
use clap::{ArgMatches, FromArgMatches};
use colored::{Colorize, Color};

#[derive(Debug)]
pub enum AttribCmd {
    New(Option<Attrib>),
    Help,
    List,
}

impl Default for AttribCmd {
    fn default() -> Self {
        Self::Help
    }
}


impl Cmd for AttribCmd {

    fn name() -> &'static str { "attrib" }
    fn about() -> &'static str { "The attribs cmd" }
    fn long_about() -> &'static str { "The attribs cmd" }
    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("help")
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            clap::App::new("new"),
            Self::list_cmd(),
            Self::help_cmd(),
        ]
    }

    fn run(&self) {
        println!("{}", format!("Running attrib cmd...")
            .color(Color::BrightRed))

    }

    fn print_help() {
        let help = format!("
            ATTRIB: The attribute command allows for the look-\n
                    up or tagging of facts/items/records by u-\n
                    ser defined or automatically defined...\n
        ").color(Color::BrightRed);
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

impl AttribCmd {
    pub fn list_cmd() -> clap::App<'static> {
        clap::App::new("list")
            .about("List attributes in your log.")
            .long_about("Subcommand to list all attributes in your log, or a subset based on some criteria")
            .subcommands(vec![
                clap::App::new("fact")
                    .long_about("List all attributes given to facts"),
                clap::App::new("record")
                    .long_about("List all attributes given to records"),
                clap::App::new("item")
                    .long_about("List all attributes given to items")
            ])
            .args(&vec![
                clap::Arg::new("fact-name")
                    .short('f')
                    .long("fact")
                    .about("List attributes related to a specified fact in your log"),
                clap::Arg::new("item-name")
                    .short('i')
                    .long("item")
                    .about("List attributes related to a specified item in your log"),
                clap::Arg::new("record-name")
                    .short('r')
                    .long("record")
                    .aliases(&["rec", "log"])
                    .about("List attributes related to a specified item in your log"),
            ])

    }
}

impl FromArgMatches for AttribCmd {
    fn from_arg_matches(_matches: &ArgMatches) -> Self {
        Self::print_help();
        Self::default()
    }
}

impl clap::Subcommand for AttribCmd {
    fn from_subcommand(sub: Option<(&str, &ArgMatches)>)
        -> Option<Self>
    {
        let (sub, args) = sub.unwrap();
        if sub == "attrib" {
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
