use std::collections::HashSet;

pub fn execute() {
    let inputs = crate::start_day::setup(3);

    println!("Part 1: {}", execute_part_one(&inputs));
    println!("Part 2: {}", execute_part_two(&inputs));
}

fn execute_part_one(inputs: &Vec<String>) -> u64 {
    let mut total_priorities = 0u64;

    for line in inputs {
        let mut set_a = HashSet::new();
        let mut set_b = HashSet::new();

        let mut common: Option<char> = None;

        let (a, b) = line.split_at(line.len() / 2);

        for c in a.chars() {
            set_a.insert(c);
        }

        for c in b.chars() {
            set_b.insert(c);
            if set_a.contains(&c) {
                common = Some(c);
            }
        }

        if let Some(c) = common {
            total_priorities += get_priority(c);
        }
    }

    total_priorities
}

fn execute_part_two(inputs: &Vec<String>) -> u64 {
    let mut total_priorities = 0u64;

    let mut input_iter = inputs.iter();
    while let (Some(a), Some(b), Some(c)) =
        (input_iter.next(), input_iter.next(), input_iter.next())
    {
        let mut set_a: HashSet<char> = HashSet::new();
        let mut set_b: HashSet<char> = HashSet::new();
        let mut common: Vec<char> = Vec::new();

        for ch in a.chars() {
            set_a.insert(ch);
        }

        for ch in b.chars() {
            set_b.insert(ch);
            if set_a.contains(&ch) {
                common.push(ch);
            }
        }

        for ch in common {
            if c.contains(ch) {
                total_priorities += get_priority(ch);
                break;
            }
        }
    }

    total_priorities
}

fn get_priority(c: char) -> u64 {
    (match c {
        'a'..='z' => ((c as u8) - b'a') + 1,
        'A'..='Z' => ((c as u8) - b'A') + 27,
        _ => 0u8,
    }) as u64
}
