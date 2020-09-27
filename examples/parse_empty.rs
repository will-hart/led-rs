use led_rs::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
    // Get the file path
    let mut file_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("examples");
    file_path.push("empty");
    file_path.set_extension("json");

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{:?}", Project::parse_json(contents).unwrap());
}
