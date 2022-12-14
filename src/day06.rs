use std::collections::{HashSet, VecDeque};
use std::str::Chars;
use std::iter::Enumerate;

pub fn execute() {
    let input = crate::start_day::setup("06");

    let chars = input[0].chars().enumerate();

    println!("Part 1: {}", check_window_of_size(chars.clone(), 4));
    println!("Part 2: {}", check_window_of_size(chars.clone(), 14));
}

fn check_window_of_size(mut chars: Enumerate<Chars>, n: usize) -> usize {
    let mut marker_window: VecDeque<(usize, char)> = VecDeque::new();

    while let Some(ic) = chars.next() {
        marker_window.push_back(ic);

        if marker_window.len() < n {
            continue;
        }

        let mut charset: HashSet<char> = HashSet::new();
        let mut valid = true;

        for (_, c) in &marker_window {
            if charset.contains(c) {
                valid = false;
                break;
            }
            charset.insert(*c);
        }

        if valid {
            break;
        }

        marker_window.pop_front();
    }

    marker_window.pop_back().unwrap().0 + 1
}
