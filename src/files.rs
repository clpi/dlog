use dirs_next::data_local_dir;
use csv::{ReaderBuilder, Writer};
use std::{result::IntoIter, io, process, fs, thread, sync::{Arc, Mutex}};
use crate::cmd::DataType;

pub trait Writeable: DataType {

    fn read<T>(self) -> csv::Result<Vec<Vec<String>>>
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
            .from_path(self.path())?;
        let _head = rdr.headers()?;
        let mut out: Vec<Vec<String>> = Vec::new();
        while let Some(record) = rdr.records().into_iter().next() {
            let mut row: Vec<String> = Vec::new();
            record?.into_iter().for_each(|field| {
                row.push(field.to_string());
            });
            out.push(row);
        };
        Ok(out)
    }

    fn write<T>(self, input: T) -> csv::Result<()>
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
            .from_path(self.clone().path())?;
        write.write_record(vec![
            Self::cmd_string()[0],
            self.to_string().as_str(),
        ])?;
        Ok(())
    }

    fn read_records<R>(_reader: R) -> csv::Result<()> {
        Ok(())
    }

}
