pub mod fact;
pub mod item;
pub mod record;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type Err = Box<dyn std::error::Error>;

#[test]
fn item_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    cmd.arg("item");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("item"));
    Ok(())
}


#[test]
fn record_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    cmd.arg("record");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("record"));
    Ok(())
}

#[test]
fn fact_cmd_no_args_prints_help() -> Result<(), Err> {
    let mut cmd = Command::cargo_bin("dlog")?;
    cmd.arg("fact");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("fact"));
    Ok(())
}
