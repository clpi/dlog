use clap::FromArgMatches;
use dlog_lib::{
    cmd::{DApp, Cmd, Subcmd,
        fact::FactCmd,
    },
    models::{
        fact::{Fact, FactValue, AbstractFact},
        units::Units,
    }
};

pub fn fact_cmd_no_key_prompts_user() {

}

#[test]
pub fn fact_cmd_first_pos_gives_fact_name() -> Result<(), String> {
    let cmd = DApp::run_cmd("dlog sleep");
    if let Ok(cmd) = cmd {
        match cmd.subcmd {
            Subcmd::Fact(FactCmd::New(f, af)) => {
                debug_assert_eq!(f.name.as_str(), "sleep");
                Ok(())
            },
            _ => Err("Not".to_string())

            }
    } else {
        Err("NO".to_string())
    }

}

#[test]
pub fn fact_cmd_second_pos_gives_fact_val() -> Result<(), String> {
    let cmd = DApp::run_cmd("dlog sleep 5");
    if let Ok(cmd) = cmd {
        match cmd.subcmd {
            Subcmd::Fact(FactCmd::New(f, af)) => {
                debug_assert_eq!(f.name, "sleep".to_string());
                debug_assert_eq!(f.val, FactValue::RealNumber(5.0));
                Ok(())
            },
            _ => Err("Not".to_string())

            }
    } else {
        Err("NO".to_string())
    }
}
