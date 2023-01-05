use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::str;

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
        let file_md5 = self.collect_md5();
        let input_md5 = self.parse_supplied_md5();
        input_md5.iter().for_each(|(k, v)| {
            if let Some(md5) = file_md5.get(k) {
                if md5 == v {
                    println!("{}: OK", k);
                } else {
                    println!("{}: FAIL", k);
                }
            } else {
                println!("{}: {}: NOT FOUND", k, v);
            }
        });
    }

    fn collect_md5(&self) -> HashMap<String, String> {
        let paths = file::find_files(&self.regex);
        paths.iter().for_each(|path| {
            println!("File founds {:?}", path);
        });
        let mut md5s = HashMap::new();
        paths.iter().for_each(|path| {
            let md5 = self.check_md5(&path);

            let fname = path.file_name().unwrap().to_string_lossy().to_string();
            md5s.insert(fname, md5);
        });

        md5s
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
