use rayon::prelude::*;
use std::collections::HashMap;
use std::iter;
use std::path::Path;
use std::process::Command;
use std::str;
use std::sync::atomic::AtomicI32;

use crate::file;

pub fn run_md5sum(input: &Path) {
    let output = Command::new("md5sum")
        .arg(input)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", str::from_utf8(&output.stdout).unwrap());
    println!("stderr: {}", str::from_utf8(&output.stderr).unwrap());
}

pub struct Md5<'a> {
    pub input: &'a Path,
    pub regex: &'a str,
}

impl<'a> Md5<'a> {
    pub fn new(input: &'a Path, regex: &'a str) -> Self {
        Self { input, regex }
    }

    pub fn match_md5(&self) {
        let origin_md5 = self.parse_supplied_md5();
        println!("MD5 supplied files: {}", origin_md5.len());
        let divider = iter::repeat('-').take(50).collect::<String>();
        println!("{}", divider);
        self.collect_md5(&origin_md5);
    }

    fn collect_md5(&self, origin_md5: &HashMap<String, String>) {
        let paths = file::find_files(&self.regex);
        let success_count = AtomicI32::new(0);
        let failed_count = AtomicI32::new(0);
        let not_found_count = AtomicI32::new(0);
        paths.par_iter().for_each(|path| {
            let md5 = self.check_md5(&path);

            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            if let Some(origin_md5) = origin_md5.get(&fname) {
                if origin_md5.to_string() == md5 {
                    println!("{}: OK", fname);
                    success_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                } else {
                    println!("{}: FAIL", fname);
                    failed_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            } else {
                println!("Path {}: MD5 {}: NOT FOUND", fname, md5);
                not_found_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        });
        println!(
            "{}: {}",
            "SUCCESS",
            success_count.load(std::sync::atomic::Ordering::Relaxed)
        );
        println!(
            "{}: {}",
            "FAILED",
            failed_count.load(std::sync::atomic::Ordering::Relaxed)
        );
        println!(
            "{}: {}",
            "NOT FOUND",
            not_found_count.load(std::sync::atomic::Ordering::Relaxed)
        );
        println!("DONE");
    }

    fn check_md5(&self, path: &Path) -> String {
        let output = Command::new("md5sum")
            .arg(path)
            .output()
            .expect("Error: md5sum not found");

        match str::from_utf8(&output.stdout) {
            Ok(s) => {
                let md5 = s.split_whitespace().collect::<Vec<&str>>();
                md5[0].to_string()
            }
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }
    }

    fn parse_supplied_md5(&self) -> HashMap<String, String> {
        let paths = file::find_files("txt");
        let mut md5s = HashMap::new();
        paths.iter().for_each(|path| {
            let lines = file::parse_file(&path);
            lines.iter().for_each(|line| {
                let md5 = line.split_whitespace().collect::<Vec<&str>>();
                md5s.insert(md5[1].to_string(), md5[0].to_string());
            });
        });

        md5s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_md5() {
        let md5 = Md5::new(&Path::new("test"), "txt");
        let test = md5.check_md5(Path::new("tests/files/test.txt"));
        assert_eq!("d41d8cd98f00b204e9800998ecf8427e", test);
    }
}
