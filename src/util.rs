use std::io::{Read, self};

pub fn get_input() -> io::Result<String> {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp)?;
    Ok(inp)
}
