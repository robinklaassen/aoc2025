use std::io::BufRead;
use std::{fs::File, io::BufReader};

pub mod direction;
pub mod grid;

/// Read a file path into a vector of strings, without newline characters
pub fn read_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("should be able to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("should be able to parse line"))
        .collect()
}
