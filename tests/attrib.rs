use dlog_lib::cmd::{DApp, Cmd, fact::FactCmd, attribute::AttribCmd, Subcmd};
use dlog_lib::models::{
    fact::{Fact, FactValue},
    attrib::Attrib,
};

#[test]
pub fn entry_linked_attrib_from_lc_a_flag() -> Result<(), String> {
    let short = DApp::run_cmd("dlog sleep 5 hr -a dreamt");
    let long = DApp::run_cmd("dlog sleep 5 hr --attrib dreamt");
    if let (Ok(s), Ok(l)) = (short, long) {
        match (s.subcmd, l.subcmd) {
            (Subcmd::Fact(FactCmd::New(fs, afs)),
             Subcmd::Fact(FactCmd::New(fl, afl))) => {
                let sa = Attrib::from("dreamt".to_string());
                debug_assert_eq!(fs.attribs, vec![sa.clone()]);
                debug_assert_eq!(fl.attribs, vec![sa.clone()]);
                debug_assert_ne!(afs.attribs, vec![sa.clone()]);
                debug_assert_ne!(afl.attribs, vec![sa.clone()]);
                Ok(())
            },
            _ => Err("Not".to_string())
        }
    } else {
        Err("NO".to_string())
    }
}

#[test]
pub fn fact_type_linked_attrib_from_up_a_flag() -> Result<(), String> {
    let short = DApp::run_cmd("dlog sleep 5 hr -A health");
    let long = DApp::run_cmd("dlog sleep 5 hr --link-attrib health");
    if let (Ok(s), Ok(l)) = (short, long) {
        match (s.subcmd, l.subcmd) {
            (Subcmd::Fact(FactCmd::New(fs, afs)),
             Subcmd::Fact(FactCmd::New(fl, afl))) => {
                let sa = Attrib::from("health".to_string());
                debug_assert_eq!(afs.attribs, vec![sa.clone()]);
                debug_assert_eq!(afl.attribs, vec![sa.clone()]);
                debug_assert_ne!(fs.attribs, vec![sa.clone()]);
                debug_assert_ne!(fl.attribs, vec![sa.clone()]);
                Ok(())
            },
            _ => Err("Not".to_string())

            }
    } else {
        Err("NO".to_string())
    }
}

pub fn attrib_with_equals_gives_val() -> Result<(), String> {
    let short = DApp::run_cmd("dlog nap -a at=home");
    let long = DApp::run_cmd("dlog nap 5 hrs -A mood=cranky");
    if let (Ok(s), Ok(l)) = (short, long) {
        match (s.subcmd, l.subcmd) {
            (Subcmd::Fact(FactCmd::New(fs, afs)),
             Subcmd::Fact(FactCmd::New(fl, afl))) => {
                let sa = Attrib::from("health".to_string());
                debug_assert_eq!(afs.attribs, vec![sa.clone()]);
                debug_assert_eq!(afl.attribs, vec![sa.clone()]);
                debug_assert_ne!(fs.attribs, vec![sa.clone()]);
                debug_assert_ne!(fl.attribs, vec![sa.clone()]);
                Ok(())
            },
            _ => Err("Not".to_string())

            }
    } else {
        Err("NO".to_string())
    }
}
