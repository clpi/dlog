use std::{io, path::PathBuf};

pub fn create<P: Into<PathBuf>>(path: P) -> io::Result<()> {
    let mut wtr = csv::Writer::from_path(&path.into().as_path())?;
    wtr.write_record(&[
        "City",
        "State",
        "Population",
        "Latitude",
        "Longitude",
    ])?;
    wtr.write_record(&[
        "Davidsons Landing",
        "AK",
        "",
        "65.2419444",
        "-165.2716667",
    ])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;
    wtr.flush()?;

    Ok(())
}

pub fn read<P: Into<PathBuf>>(path: P) -> io::Result<()> {
    let mut rdr =
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(path.into())?;
    Ok(())
}
