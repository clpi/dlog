pub mod app;
pub mod user;
pub mod cmd;

pub fn run() -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
