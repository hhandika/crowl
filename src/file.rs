use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn parse_file(input: &Path) -> Vec<String> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let divider = iter::repeat('-').take(50).collect::<String>();
    println!("{}", divider);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| {
            lines.push(line);
        });
    lines
}

pub fn walk_dir(regex: &str) -> HashMap<String, PathBuf> {
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

            if re_match_lazy(regex, &ext.to_string_lossy()) {
                let file_name = e.file_name().to_string_lossy().to_string();
                let path = e.path().to_path_buf();
                paths.insert(file_name, path);
            }
        });
    paths
}

pub fn find_files(regex: &str) -> Vec<PathBuf> {
    let mut fnames = Vec::new();
    WalkDir::new(".")
        .into_iter()
        .filter_map(|ok| ok.ok())
        .filter(|e| e.file_type().is_file())
        .for_each(|e| {
            let fname = e.file_name().to_string_lossy();
            if re_match_lazy(regex, &fname) {
                fnames.push(PathBuf::from(e.path()));
            }
        });
    fnames
}

fn re_match_lazy(regex: &str, fname: &str) -> bool {
    let re = regex::Regex::new(regex).unwrap();
    re.is_match(fname)
}
