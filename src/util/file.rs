use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::{io, path};

pub fn all_files_in_data_dir<P: AsRef<path::Path>>(
    paths: &[P], ignore: &[&str],
) -> io::Result<()> {
    let mut walker = WalkBuilder::new(&paths[0]);
    let mut overrides = OverrideBuilder::new(&ignore[0]);
    while let Some(path) = paths[1..].as_ref().iter().next() {
        walker.add(path);
    }
    while let Some(ignored) = ignore.iter().next() {
        overrides.add(format!("!{}", ignored).as_str())
            .expect("Could not add override");
    }
    let overrides = overrides.build().expect("Invalid ignore directories");
    walker.overrides(overrides);
    walker.build_parallel().run(move || {
        Box::new(move |entr| {
            match entr {
                Err(_e) => ignore::WalkState::Quit,
                Ok(_entr) => ignore::WalkState::Continue,
            }
        })
    });
    Ok(())
}
