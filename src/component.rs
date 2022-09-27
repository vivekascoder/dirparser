use std::path::Path;

pub fn main() {
    // How does the component thing works?
    let p = Path::new("/code/rust_cookbook/README.md");
    let c = p.components();

    println!("Components: {:?}", c);
}
