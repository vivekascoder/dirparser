# DirParser

Dead simple `tree` alternative written in rust that respects `.gitignore`.

## Example

```bash
dirparser .
```

## Resources

- https://doc.rust-lang.org/std/result/

```rs
use glob::glob;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn is_excluded(gitignore_path: &str, p: &Path) -> bool {
    let content = fs::read_to_string(gitignore_path).expect("Gitignore file can't be loaded.");
    let mut result: Vec<_> = content
        .as_str()
        .split("\n")
        .filter(|line| {
            if line.starts_with("#") {
                false
            } else if line.is_empty() {
                false
            } else {
                true
            }
        })
        .collect();
    result.push("./.git");

    for pattern in result {
        if is_match(pattern, p.to_str().unwrap()) {
            return true;
        }
    }
    return false;
}

fn is_match(pattern: &str, val: &str) -> bool {
    let mut p: String;
    let v: String;
    if pattern.starts_with("/") {
        p = format!(".{}", pattern);
    } else if pattern.starts_with("./") {
        p = format!("{}", pattern);
    } else if pattern.starts_with("*") {
        p = format!("./{}", pattern);
    } else {
        p = format!("{}", pattern);
    }

    if val.starts_with("./") {
        v = val.replace("./", "");
    } else {
        v = format!("{}", val);
    }

    // Match pattern.
    let s = glob(&p).unwrap().map(|f| f.unwrap()).find(|f| {
        &v == f.to_str().unwrap()
    });

    match s {
        Some(some) => {
            if some.to_str().unwrap().len() > 0 {
                return true;
            } else {
                false
            }
        }
        None => {
            return false;
        }
    }
}

fn recursive_parse<'a>(
    p: &'a str,
    ignore: &str,
    dirs: &'a mut Vec<PathBuf>,
) -> Result<&'a mut Vec<PathBuf>, std::io::Error> {
    // Iterate over files.
    for file in fs::read_dir(p)? {
        let dir_entry = &file?;

        // If the file is not ignored.
        if !(is_excluded(ignore, dir_entry.path().as_path())) {
            // If it's a dir recursively call.
            if dir_entry.path().is_dir() {
                // If it's a directory.
                recursive_parse(dir_entry.path().as_path().to_str().unwrap(), ignore, dirs);
            } else {
                dirs.push(dir_entry.path());
            }
        }
    }

    Ok(dirs)
}

fn main() -> Result<(), std::io::Error> {
    let mut dirs: Vec<PathBuf> = vec![];

    // Parse the gitignore
    let gitignore_path = "./.gitignore";

    let results = recursive_parse("./", &gitignore_path, &mut dirs)?;
    for i in results {
        println!("+ {}", i.to_str().unwrap());
    }
    Ok(())
}

```
