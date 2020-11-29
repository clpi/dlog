use assert_cmd::{
    prelude::*,
    cmd::Command, output::{OutputOkExt, OutputResult},
    assert::{Assert, OutputAssertExt},
};
use pico_args::{Arguments, Error};

pub fn two_subcmds_gives_field_and_value() -> OutputResult {
    Command::cargo_bin("main").unwrap()
        .arg("sleep").arg("6")
        .ok()
}

pub fn three_subcmds_give_field_value_units() -> OutputResult {
    Command::cargo_bin("main").unwrap()
        .arg("sleep").arg("6").arg("hr").ok()
}

pub fn shorthand_works() -> OutputResult {
    Command::cargo_bin("main").unwrap()
        .arg("sleep").arg("6").arg("hr").ok()
}
