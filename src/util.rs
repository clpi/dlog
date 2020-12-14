use colored::{Colorize, Color};
use std::{fs, io::{Read, self}, path::PathBuf};

pub fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let mut inp = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut inp)?;
    inp = inp.trim().to_string();
    Ok(inp)
}

pub fn prompt_input(prompt: &str) -> io::Result<String> {
    let name = dialoguer::Input::new()
        .with_prompt(prompt)
        .allow_empty(false)
        .validate_with(|input: &String| -> Result<(), &str> {
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
        })
        .interact()
        .expect("Could not read user input");
    println!("{}", format!("Got new item: {}", &name)
        .color(Color::BrightCyan));
    Ok(name)

}

pub fn create_dir(parent: PathBuf, name: &str) -> io::Result<PathBuf> {
    let dir = fs::create_dir(parent.join(name).as_path())?;
    Ok(parent.join(name))
}

pub fn create_default_conf() -> io::Result<()> {
    fs::copy("../assets/Config.default.toml", crate::config::Config::conf_dir())?;
    Ok(())
}

pub fn tokenize(input: String) -> Vec<String> {
    let tokens: Vec<String> = input.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    tokens
}

pub fn write_file(input: String, path: PathBuf) -> io::Result<fs::File> {
    use io::Write;
    let mut file = fs::File::create(path.as_path())?;
    write!(file, "{}", input)?;
    Ok(file)
}

pub fn get_or_create_conf_dir() -> io::Result<PathBuf> {
    let def_conf_dir = dirs_next::config_dir()
        .expect("Couldn't find default config dir")
        .join("dlog");
    if !def_conf_dir.exists() || !def_conf_dir.is_dir() {
        fs::create_dir(&def_conf_dir)?;
    }
    Ok(def_conf_dir)
}

pub fn get_or_create_data_dir() -> io::Result<PathBuf> {
    let def_data_dir = dirs_next::data_dir()
        .expect("Could not find default data dir")
        .join("dlog");
    if !def_data_dir.exists() || !def_data_dir.is_dir() {
        fs::create_dir(&def_data_dir)?;
    }
    Ok(def_data_dir)
}

