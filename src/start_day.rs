use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let f = File::open(filename)?;
    Ok(io::BufReader::new(f).lines())
}

fn print_day(day: usize) {
    println!("================== DAY {} ==================", day);
}

pub fn setup(day: usize) -> io::Lines<io::BufReader<File>> {
    print_day(day);
    read_lines(format!("inputs/{}.txt", day))
        .expect(&format!("Couldn't load inputs for {}", day))
}
