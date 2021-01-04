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
    // let cmd = assert_cmd::cmd::Command::cargo_bin("dlog")
    //     .unwrap()
    //     .arg("sleep")
    //     .assert()
    //     .success();
    let cmd = DApp::new_from(vec![ "dlog", "sleep" ].iter());
    if let Ok(cmd) = cmd {
        match cmd.subcmd {
            Subcmd::Fact(FactCmd::New(f)) => {
                assert_eq!(f.name.as_str(), "sleep");
                Ok(())
            },
            _ => Err("Not".to_string())

            }
    } else {
        Err("NO".to_string())
    }

}
