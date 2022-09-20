use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use clap::{crate_description, crate_name, crate_version, Arg, Command};

fn main() {
    let input = parse_cli();
    let file_list = parse_file(&input);
    let all_files = walk_dir();
    file_list.iter().for_each(|file| {
        if !all_files.contains_key(file) {
            println!("{} not found", file);
        } else {
            println!("{} found", file);
        }
    });
}

fn parse_cli() -> PathBuf {
    let arg = Command::new(crate_name!())
        .version(crate_version!())
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(crate_description!())
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();
    let input = PathBuf::from(arg.value_of("input").unwrap());
    input
}

fn parse_file(input: &Path) -> Vec<String> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            lines.push(line);
        });
    lines
}

fn walk_dir() -> HashMap<String, PathBuf> {
    let mut paths = HashMap::default();
    WalkDir::new(".")
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let ext = match e.path().extension() {
                Some(ext) => ext,
                None => return,
            };

            if ext == "JPG" || ext == "jpg" {
                let file_name = e.file_name().to_string_lossy().to_string();
                let path = e.path().to_path_buf();
                paths.insert(file_name, path);
            }
        });
    paths
}
