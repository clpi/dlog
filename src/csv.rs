use std::{fs, io, path::PathBuf};

pub fn create<P: Into<PathBuf>>(path: P) -> io::Result<()> {
    let mut wtr = csv::Writer::from_path(&path.into().as_path())?;
    wtr.write_record(&["a"])?;
    wtr.flush()?;

    Ok(())
}

pub fn read<P: Into<PathBuf>>(path: P) -> io::Result<()> {
    let mut _rdr =
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path.into())?;
    Ok(())
}

pub fn csv_reader<P: Into<PathBuf>>(path: P)
    -> io::Result<csv::Reader<fs::File>>
{
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .double_quote(false)
        .escape(Some(b'\\'))
        .from_path(&path.into())?;
    Ok(rdr)
}

pub fn csv_writer<P, I>(path: P, item: I) -> io::Result<csv::Writer<fs::File>>
    where
        P: Into<PathBuf>,
        I: serde::Serialize,
{
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(&path.into())?;
    wtr.serialize(item)?;
    wtr.flush()?;
    Ok(wtr)
}

