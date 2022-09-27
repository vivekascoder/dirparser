mod component;

use glob::glob;
use std::{
    fs::{self, DirEntry},
    path::{Component, Path, PathBuf},
};

fn is_excluded(gitignore_path: &str, p: &Path) -> bool {
    let content = fs::read_to_string(gitignore_path).unwrap();
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
/**
 * if 0
 * then
 * |-
 * else
 * loop
 * |  |  |-
 */

fn give_prefix_char(l: &usize) -> String {
    let mut s = String::from("");

    for i in 0..*l {
        if i == (*l - 1) {
            s.push_str("├");
        } else {
            s.push_str("│");
            s.push_str("   ");
        }
    }
    return s;
}

fn is_match(pattern: &str, val: &str) -> bool {
    let p: String;
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
    let s = glob(&p)
        .unwrap()
        .map(|f| f.unwrap())
        .find(|f| &v == f.to_str().unwrap());

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

fn pndp<'a>(dirs: &'a mut Vec<PathBuf>, dir: &DirEntry, level: usize) {
    let width = 2;
    dirs.push(dir.path());

    if dir.path().is_dir() {
        println!(
            "{}{} {}",
            give_prefix_char(&(level + 1)),
            "─".repeat(level + 1 * width),
            dir.path().as_path().to_str().unwrap()
        );
    } else {
        println!(
            "{}{} {}",
            give_prefix_char(&(level + 1)),
            "─".repeat(width),
            match dir
                .path()
                .as_path()
                .components()
                .collect::<Vec<Component>>()
                .pop()
                .unwrap()
            {
                Component::Normal(v) => v.to_str().unwrap(),
                Component::Prefix(p) => p.as_os_str().to_str().unwrap(),
                Component::RootDir => "/",
                Component::CurDir => "./",
                Component::ParentDir => "../",
            }
        );
    }
}

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
        if !(is_excluded(ignore, dir_entry.path().as_path())) {
            // If it's a dir recursively call.
            if dir_entry.path().is_dir() {
                // If it's a directory.
                pndp(dirs, dir_entry, l);

                recursive_parse(
                    dir_entry.path().as_path().to_str().unwrap(),
                    ignore,
                    dirs,
                    Some(l + 1),
                )
                .unwrap();
            } else {
                pndp(dirs, dir_entry, l);
            }
        }
    }

    Ok(dirs)
}

fn main() -> Result<(), std::io::Error> {
    let mut dirs: Vec<PathBuf> = vec![];

    // Parse the gitignore
    let gitignore_path = "./.gitignore";

    let results = recursive_parse("./", &gitignore_path, &mut dirs, None)?;
    Ok(())
}
