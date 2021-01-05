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

    fn name() -> &'static str { "action" }
    fn about() -> &'static str { "The action cmd" }
    fn long_about() -> &'static str { "The action cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        vec![
            clap::Arg::new("name")
        ]
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            clap::App::new("new"),
            clap::App::new("if")
                .alias("when")
                .short_flag('i')
                .about("Create a conditional for some object")
                .long_about("Subcommand to create a conditional action")
                .subcommands(vec![
                    clap::App::new("equals")
                        .aliases(&vec!["=", "==", "===", "eq", "true"])
                        .about("Compares the following arguments fore equality")
                        .long_about("Comares following two positional arguments, as objects, for some equality")
                        .args(&vec![
                            clap::Arg::new("TARGET")
                                .index(1)
                                .required(true)
                        ]),
                    clap::App::new("not")
                    .aliases(&vec!["!", "~", "false", "not-equals", "not-true"],)
                    .about("Negates whatever condition comes after")
                    .long_about("Negates following condition")
                    .args(vec![
                        clap::Arg::new("OBJECT")
                            .index(1)
                            .required(true)
                    ]),
                    clap::App::new("cmd")
                        .aliases(&vec!["command", "op", "function"])
                        .about("Add a new command to the databas, or do other operations")
                        .long_about("Set a command to a conditional, or add/delete a command, or check current commands attached to a conditional")
                ])
                .args(vec![
                    clap::Arg::new("OBJECT")
                        .index(1),
                    clap::Arg::new("CONDITIONAL")
                        .about("Natural language positional conditional arg")
                        .long_about("Provide a conditional to compare two different objects")
                        .possible_values(&vec![
                            "equals", "=", "==", "===", "!=", "~", "<", ">", "<=", ">=",
                            "is", "isnt", "was", "has", "will",
                        ]),
                    clap::Arg::new("equals")
                        .short('e')
                        .short_alias('=')
                        .about("Compares the preceding argument for equality with this value")
                        .long_about("Comares preceding positional arguments, as objects, for some equality with the value of this arg. May provide an expression instead of simply just an object name")
                        .value_name("TARGET")
                        .long("equals")
                        .multiple_occurrences(true)
                        .requires("OBJECT")
                        .aliases(&vec!["eq", "is", "==", "=", "==="],),
                    clap::Arg::new("not")
                        .short('n')
                        .short_aliases(&vec!['~', '!'])
                        .aliases(&vec!["is-not", "isnt", "not-equal", "ne"])
                        .long("not")
                        .about("Compares the preceding argument")
                        .long_about("Compares the preceding argument for equality with this value")
                        .requires("OBJECT")
                        .value_name("OBJECT")
                        .multiple_occurrences(true)

                ]),
            Self::help_cmd(),
        ]
    }

    fn run(&self) {
        println!("{}", format!("Running action cmd...")
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

