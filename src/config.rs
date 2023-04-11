use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_config(filename: &str) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines
}
