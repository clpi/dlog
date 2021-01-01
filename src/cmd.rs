pub mod fact;
pub mod action;
pub mod item;
pub mod record;
pub mod attribute;
pub mod link;
pub mod user;
pub mod stats;
pub mod args;

use super::config::DConfig;
use self::{
    item::ItemCmd,
    fact::FactCmd,
    record::RecordCmd,
    link::LinkCmd,
    attribute::AttribCmd,
    user::UserCmd,
    stats::StatsCmd,
    action::ActionCmd,
};
use colored::{Color, Colorize};
use clap::{Arg, ArgMatches, Clap, FromArgMatches};

pub enum App {
    Record(RecordCmd),
    Item(ItemCmd),
    Help,
}

impl Default for App {
    fn default() -> Self {
        Self::Help
    }
}

pub struct TermSettings {
    atty: bool,
    color: clap::AppSettings,
}

impl TermSettings {
    pub fn new() -> Self {
        let color = match std::env::var_os("NO_COLOR") {
            Some(_) => clap::AppSettings::ColoredHelp,
            None => clap::AppSettings::ColorNever,
        };
        let atty = atty::is(atty::Stream::Stdout);
        Self { atty, color }
    }
}

impl Cmd for App {

    fn name() -> &'static str { "dlog" }
    fn about() -> &'static str { "" }
    fn long_about() -> &'static str { "The action cmd" }

    fn args() -> Vec<clap::Arg<'static>> {
        let mut args = FactCmd::args();
        // let mut args: Vec<Arg> = Vec::new();
        args.extend(vec![
            Self::version(),
            Self::output(),
            Self::config_file(),
            clap::Arg::new("pretty-print")
                .about("Print output into a visually pleasing style")
                .takes_value(false)
                .short('p')
                .long("pretty"),
        ]);
        args
    }

    fn subcmds() -> Vec<clap::App<'static>> {
        vec![
            ItemCmd::cmd(),
            RecordCmd::cmd(),
            FactCmd::cmd(),
            AttribCmd::cmd(),
            LinkCmd::cmd(),
            StatsCmd::cmd(),
            UserCmd::cmd(),
            ActionCmd::cmd(),
            Self::help_cmd(),
            clap::App::new("init")
                .about("Initialize a fact database in the current folder"),
            clap::App::new("export")
                .about("Export all of your data to a .zip file or HTML, or save your data to a file to be imported later"),
            clap::App::new("import")
                .about("Import dlog data or other data sources into a local Dlog database"),
            clap::App::new("inbox")
                .about("Show operations related to unorganized facts and items"),
        ]
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

    fn cmd() -> clap::App<'static> {
        let term = TermSettings::new();
        let _conf = DConfig::load();
        clap::app_from_crate!()
            .setting(term.color)
            .setting(clap::AppSettings::DeriveDisplayOrder)
            .subcommands(Self::subcmds())
            .args(Self::args())
            .subcommands(Self::subcmds())
            .setting(term.color)
            .setting(clap::AppSettings::DeriveDisplayOrder)
    }

    fn run(&self) {
        let matches = Self::cmd()
            .get_matches();
        let _app = Self::from_arg_matches(&matches);
    }

}

impl FromArgMatches for App {
    fn from_arg_matches(matches: &ArgMatches) -> Self {
        match matches.subcommand() {
            Some(("record", sub)) => RecordCmd::from_arg_matches(sub).run(),
            Some(("item", sub)) => ItemCmd::from_arg_matches(sub).run(),
            Some(("fact", sub)) => FactCmd::from_arg_matches(sub).run(),
            Some(("link", sub)) => LinkCmd::from_arg_matches(sub).run(),
            Some(("attrib", sub)) => AttribCmd::from_arg_matches(sub).run(),
            Some(("user", sub)) => UserCmd::from_arg_matches(sub).run(),
            Some(("stats", sub)) => StatsCmd::from_arg_matches(sub).run(),
            Some((&_, &_)) => {},
            None => {
                println!("subc: {:#?}\n matches: {:#?}",
                    matches.subcommand(),
                    matches
                );
            }
        }
        Self::default()
    }
}

impl App {


    pub fn version() -> Arg<'static> {
        clap::Arg::new("version")
            .long("version")
            .about("Print version info")
            .takes_value(false)
    }

    pub fn output() -> Arg<'static> {
        clap::Arg::new("output")
            .short('o')
            .long("output")
            .about("Print version info")
            .takes_value(true)
    }

    pub fn match_output(val: String) {
        match val.as_str() {
            "json" => { println!("JSON output") },
            "yaml" => { println!("YAML output") }
            _ => { println!("Invalid output type") }
        }
    }

    pub fn config_file() -> Arg<'static> {
        clap::Arg::new("config")
            .short('c')
            .long("config")
            .about("Manually set config file location and load")
            .takes_value(true)
    }

    pub fn set_config() -> Arg<'static> {
        clap::Arg::new("set")
            .short('s')
            .long("set")
            .about("Set a config key-value pair manually")
            .exclusive(true)
            .takes_value(true)
    }

    pub fn matches() -> () {
    }

    pub fn print_help() {
        println!("dlog help")
    }

    pub fn help_cmd() -> clap::App<'static> {
        clap::App::new("base_help")
            .about("Prints the help for dlog")
            .long_about("Prints help of dlog with no args, otherwise, input 'fact', 'record', 'item', etc. for a summary of how to use these different subcommands")
            .short_flag('h')
            .long_flag("help")
    }
}

pub trait Cmd: FromArgMatches + Default {
    fn run(&self);
    fn cmd() -> clap::App<'static> {
        clap::App::new(Self::name())
            .about(Self::about())
            .long_about(Self::long_about())
            .subcommands(Self::subcmds())
            .args(Self::args())
    }
    fn name() -> &'static str;
    fn about() -> &'static str;
    fn long_about() -> &'static str;
    fn args() -> Vec<clap::Arg<'static>>;
    fn subcmds() -> Vec<clap::App<'static>>;
    fn print_help();
    fn help_cmd() -> clap::App<'static>;
}


#[derive(Clap, Debug)]
pub enum Command {
    Item,
    Record,
    Fact,
    Link,
    Attrib,
    Free,
}

impl std::str::FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "item" => Command::Item,
            "record" => Command::Record,
            "fact" => Command::Fact,
            "link" => Command::Link,
            "attrib" => Command::Attrib,
            _ => Command::Free,
        };
        Ok(res)
    }
}


