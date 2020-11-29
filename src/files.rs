use csv::{Reader, Writer};
use std::{io, process};

pub struct FileIo {

}

impl FileIo {

    pub fn new(fields: &str) -> io::Result<Self> {
        let mut rdr = csv::Reader::from_reader(io::stdin());
        for res in rdr.records() {
            match res {
                Ok(rec) => println!("{:?}", rec),
                Err(err) => println!("No good"),
            }
        };
        Ok(Self {  })
    }

}
