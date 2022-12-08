use std::iter::Enumerate;
use std::slice::Iter;

pub fn execute() {
    let inputs = crate::start_day::setup(5);

    let mut stacks: Vec<Vec<char>> = Vec::new();

    let mut lines = inputs.iter().enumerate();

    while let Some((i, s)) = lines.next() {
        if s.len() == 0 {
            let mut backlines = inputs.iter().take(i).rev();

            if let Some(s) = backlines.next() {
                for _ in 0..((s.len() - 1) / 4 + 1) {
                    stacks.push(Vec::new());
                }
            }

            while let Some(s) = backlines.next() {
                let s = "..".to_string() + s;
                let mut chars = s.chars();
                let mut stack = 0;
                while let Some(c) = chars.nth(3) {
                    match c {
                        'A'..='Z' => stacks[stack].push(c),
                        _ => (),
                    }
                    stack += 1;
                }
            }

            break;
        }
    }

    part_one(lines.clone(), stacks.clone());
    part_two(lines.clone(), stacks.clone());
}

fn part_one(mut lines: Enumerate<Iter<String>>, mut stacks: Vec<Vec<char>>) {
    while let Some((_, s)) = lines.next() {
        let m: Vec<u32> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        for _ in 0..m[0] {
            if let Some(c) = stacks[m[1] as usize - 1].pop() {
                stacks[m[2] as usize - 1].push(c);
            }
        }
    }

    let mut tops: Vec<char> = Vec::new();

    for stack in stacks {
        if let Some(c) = stack.last() {
            tops.push(*c);
        }
    }

    println!("Part 1: {}", tops.iter().collect::<String>());
}

fn part_two(mut lines: Enumerate<Iter<String>>, mut stacks: Vec<Vec<char>>) {
    while let Some((_, s)) = lines.next() {
        let m: Vec<u32> = s
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let from = m[1] as usize - 1;
        let to = m[2] as usize - 1;
        let amount = stacks[from].len() - (m[0] as usize);

        let from_clone = stacks[from].clone();
        let splits = from_clone.split_at(amount);
        stacks[from] = Vec::from(splits.0);
        for c in splits.1 {
            stacks[to].push(*c);
        }
    }

    let mut tops: Vec<char> = Vec::new();

    for stack in stacks {
        if let Some(c) = stack.last() {
            tops.push(*c);
        }
    }

    println!("Part 2: {}", tops.iter().collect::<String>());
}
