use csv::{ReaderBuilder, Writer};
use std::{io, process};

pub struct CsvWriter {
}

impl CsvWriter {

    pub fn read(path: &str) -> io::Result<Self> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .double_quote(true)
            .escape(Some(b'\\'))
            .delimiter(b';')
            .comment(Some(b'#'))
            .double_quote(true)
            .from_path(path)?;
        let _head = rdr.headers()?;
        for res in rdr.records() {
            match res {
                Ok(rec) => println!("{:?}", rec),
                Err(err) => {
                }
            }
        };
        Ok(Self {  })
    }

    pub fn write(path: &str) -> io::Result<Self> {
        let mut write = csv::WriterBuilder::new()
            .has_headers(true)
            .flexible(true)
            .double_quote(true)
            .escape(b'\\')
            .delimiter(b';')
            .double_quote(true)
            .from_path(path);
        Ok(Self {  })
    }

}
