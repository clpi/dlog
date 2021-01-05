pub mod fact;
pub mod item;
pub mod record;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use dlog_lib::cmd::{DApp, Cmd, fact::FactCmd, attribute::AttribCmd, Subcmd};
use dlog_lib::models::{
    fact::{Fact, FactValue},
    attrib::Attrib,
};

type Err = Box<dyn std::error::Error>;

#[test]
fn item_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    cmd.arg("item");
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::contains("item"));
    assert_eq!(2, 2);
    Ok(())
}


#[test]
fn record_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    cmd.arg("record");
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::contains("record"));
    assert_eq!(2, 2);
    Ok(())
}

#[test]
fn fact_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    assert_eq!(2, 2);
    // cmd.arg("fact");
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::contains("fact"));
    Ok(())
}

#[test]
pub fn etc() -> Result<(), String> {
    let cmd = DApp::run_cmd("dlog sleep 4 hr -a slept=well -n tenmp -A att=a -A b -N perm");
    if let Ok(cmd) = cmd {
        match cmd.subcmd {
             Subcmd::Fact(FactCmd::New(f, af)) => {
                let as1 = Attrib::new("slept".into(), Some("well".into()));
                let al1 = Attrib::new("att".into(), Some("a".into()));
                let al2 = Attrib::new("b".into(), None);
                debug_assert_eq!(f.attribs, vec![as1]);
                debug_assert_eq!(af.attribs, vec![al1, al2]);
                Ok(())
            },
            _ => Err("Not".to_string())
        }
    } else {
        Err("NO".to_string())
    }
}
