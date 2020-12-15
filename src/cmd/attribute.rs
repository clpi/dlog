use crate::{
    util::prompt_input,
    cmd::Cmd,
};
use std::fmt;
use serde::{Serialize, Deserialize};
use clap::{Clap, ArgMatches, FromArgMatches};
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

    fn cmd() -> clap::App<'static> {
        clap::App::new("attrib")
            .about("attribs")
            .subcommands(vec![
                clap::App::new("new")
            ])
            .args(&vec![
                clap::Arg::new("name")
            ])
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Attrib {
    pub name: String,
}

impl Attrib {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    pub fn from_prompt(prompt: String) -> Vec<Attrib> {
        if prompt.len() != 0 {
            let attribs: Vec<Attrib> = prompt.split_whitespace()
                .into_iter()
                .map(|a| Attrib::from(a.to_string()))
                .collect();
            attribs
        } else { vec![] }
    }

    pub fn prompt(prompt: &str) -> Vec<Attrib> {
        let attrib = prompt_input("Attributes? (Enter if not applicable): ")
            .expect("Could not prompt fact value");
        Self::from_prompt(attrib)
    }

    pub fn join(attribs: &Vec<Self>) -> String {
        let attribs = attribs.clone();
        attribs.iter()
            .map(|a| &a.name)
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl From<String> for Attrib {
    fn from(string: String) -> Self {
        Self { name: string }
    }
}
