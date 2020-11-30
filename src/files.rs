use csv::{ReaderBuilder, Writer};
use std::{result::IntoIter, io, process};

pub struct Csv {
}

impl Csv {

    pub fn read<T>(item: &str) -> csv::Result<Vec<Vec<String>>>
    where
        T: IntoIterator {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .double_quote(true)
            .escape(Some(b'\\'))
            .delimiter(b';')
            .comment(Some(b'#'))
            .double_quote(true)
            .from_path(item)?;
        let _head = rdr.headers()?;
        let mut out: Vec<Vec<String>> = Vec::new();
        if let Some(res) = rdr.records().next() {
            out.push(vec!["".to_string()]);
        };
        Ok(out)
    }

    pub fn write<T>(item: &str, input: T) -> csv::Result<()>
    where
        T: IntoIterator,
        T::Item: AsRef<[u8]>
    {
        let mut write = csv::WriterBuilder::new()
            .has_headers(true)
            .flexible(true)
            .double_quote(true)
            .escape(b'\\')
            .delimiter(b';')
            .double_quote(true)
            .from_path(item)?;
        write.write_record(input)?;
        Ok(())
    }

    pub fn read_records<R>(reader: R) -> csv::Result<()> {
        Ok(())
    }

}
