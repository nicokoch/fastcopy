extern crate fastcopy as fcopy;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let source_files: Vec<_> = (0..2).map(|i| {
        let mut path = PathBuf::new();
        path.push("source_files/");
        path.push(i.to_string());
        let mut source_file = fs::File::create(&path).unwrap();
        write!(source_file, "Hello World!").unwrap();
        path
    }).collect();
    let target_file = "target.txt";
    for file in &source_files {
        fcopy::copy(file, target_file).unwrap();
    }
}
