use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let f = File::open(filename)?;
    Ok(io::BufReader::new(f).lines())
}
