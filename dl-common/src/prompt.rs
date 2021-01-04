use colored::{Colorize, Color};
use std::{fs, io::{Read, prelude::*, self}, path::PathBuf};

pub fn validate_input(input: String) -> Result<(), &'static str> {
    let invalid = vec!["new", "search", "list", "info"];
    let inv_sym = vec!['@', '/', '&', '^', '$', '#'];
    for ch in inv_sym {
        if input.contains(ch) {
            return Err("Invalid character");
        }
    }
    if invalid.contains(&input.as_str())
        || input.len() > 40
        || input.contains("\\") {
        Err("Not a valid input")
    } else { Ok(()) }
}

pub fn prompt(prompt: &str) -> io::Result<String> {
    let name = dialoguer::Input::new()
        .with_prompt(prompt)
        .allow_empty(false)
        .validate_with(|input: &String| -> Result<(), &str> {
            validate_input(input.into())
        })
        .interact()
        .expect("Could not read user input");
    println!("{}", format!("Got new item: {}", &name)
        .color(Color::BrightCyan));
    Ok(name)

}
