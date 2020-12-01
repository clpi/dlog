use dirs_next::data_local_dir;
use csv::{ReaderBuilder, Writer};
use std::{result::IntoIter, io, process, fs, thread, sync::{Arc, Mutex}};

pub struct Csv {
}

impl Csv {

    pub fn read_multiple(items: Vec<String>) -> csv::Result<Vec<Vec<Vec<String>>>> {
        const NUM_THREADS: usize = 4;
        let num_reads = items.len();
        let res = if num_reads < NUM_THREADS {
            (0..num_reads).map(|n| {
                Self::read::<Vec<String>>(&items[n]).unwrap()
            }).collect()
        } else {
            (0..num_reads).map(|n| {
                let items = Arc::new(Mutex::new(items.clone()));
                thread::spawn(move || {
                    Self::read::<Vec<String>>(&items.lock().unwrap()[n]).unwrap()
                }).join().expect("Could not join threads")
            }).collect()
        };
        Ok(res)
    }

    pub fn read<T>(item: &str) -> csv::Result<Vec<Vec<String>>>
    where
        T: IntoIterator {
        let item_path = data_local_dir()
            .unwrap()
            .join("dlog")
            .join(format!("{}.csv", item));
        let item = fs::File::open(&item_path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .double_quote(true)
            .escape(Some(b'\\'))
            .delimiter(b';')
            .comment(Some(b'#'))
            .double_quote(true)
            .from_reader(item);
        let _head = rdr.headers()?;
        let mut out: Vec<Vec<String>> = Vec::new();
        while let Some(_res) = rdr.records().into_iter().next() {
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

    pub fn read_records<R>(_reader: R) -> csv::Result<()> {
        Ok(())
    }

}
