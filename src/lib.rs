use glob::glob;
use std::{
    fs::{self, DirEntry},
    path::{Component, Path, PathBuf},
};

pub fn is_excluded(gitignore_path: &str, p: &Path) -> bool {
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

pub fn give_prefix_char(l: &usize) -> String {
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

pub fn is_match(pattern: &str, val: &str) -> bool {
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

pub fn pndp<'a>(dirs: &'a mut Vec<PathBuf>, dir: &DirEntry, level: usize) {
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
