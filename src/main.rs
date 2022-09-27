use std::{
    fs::{self},
    path::PathBuf,
};

fn recursive_parse<'a>(
    p: &'a str,
    ignore: &str,
    dirs: &'a mut Vec<PathBuf>,
    level: Option<usize>,
) -> Result<&'a mut Vec<PathBuf>, std::io::Error> {
    let l: usize = match level {
        Some(val) => val,
        None => 0,
    };

    // Iterate over files.
    for file in fs::read_dir(p)? {
        let dir_entry = &file?;

        // If the file is not ignored.
        if !(dirparser::is_excluded(ignore, dir_entry.path().as_path())) {
            // If it's a dir recursively call.
            if dir_entry.path().is_dir() {
                // If it's a directory.
                dirparser::pndp(dirs, dir_entry, l);

                recursive_parse(
                    dir_entry.path().as_path().to_str().unwrap(),
                    ignore,
                    dirs,
                    Some(l + 1),
                )
                .unwrap();
            } else {
                dirparser::pndp(dirs, dir_entry, l);
            }
        }
    }

    Ok(dirs)
}

fn main() -> Result<(), std::io::Error> {
    let mut dirs: Vec<PathBuf> = vec![];

    // Parse the gitignore
    let gitignore_path = "./.gitignore";

    #[allow(unused_variables)]
    let results = recursive_parse("./", &gitignore_path, &mut dirs, None)?;

    Ok(())
}
