use std::fmt::Display;
use std::path::Path;

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path> + Display,
{
    std::fs::read_to_string(&filename)
        .expect(&format!("Couldn't load file {}", &filename))
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn print_day(day: usize) {
    println!("================== DAY {} ==================", day);
}

pub fn setup(day: usize) -> Vec<String> {
    print_day(day);
    read_lines(format!("inputs/{}.txt", day))
}
