use led_rs::render_helpers::*; // pull in render data traits
use led_rs::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Parses and prints a single map file
fn parse_file(file_name: &str) -> Project {
    let mut file_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    file_path.push("examples");
    file_path.push(file_name);
    file_path.set_extension("json");

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Project::parse_json(contents).unwrap()
}

fn main() {
    let project = parse_file("minified_all_features");
    let render_data = project.to_merged_render_grid(0).unwrap();

    println!("### MAP DATA USING ITER ###");
    for (row_num, row) in render_data.rows().enumerate() {
        for (col_num, cell) in row.iter().enumerate() {
            if cell.is_empty {
                println!("{}, {}: EMPTY", row_num, col_num);
                continue;
            }

            println!(
                "{}, {}: {} @ {},{}",
                row_num, col_num, cell.tile_id, cell.atlas_pos.0, cell.atlas_pos.1
            );
        }
    }
}
