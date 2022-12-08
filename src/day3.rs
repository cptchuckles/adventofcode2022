use std::collections::HashMap;

pub fn execute() {
    let inputs = crate::start_day::setup(3);

    println!("Part 1: {}", execute_part_one(&inputs));
}

fn execute_part_one(inputs: &Vec<String>) -> u64 {
    let mut total_priorities = 0u64;

    for line in inputs {
        let mut map_a = HashMap::new();
        let mut map_b = HashMap::new();

        let mut common: Option<char> = None;

        let (a, b) = line.split_at(line.len() / 2);

        for c in a.chars() {
            if let Some(count) = map_a.insert(c, 1) {
                map_a.insert(c, count + 1);
            }
        }

        for c in b.chars() {
            if let Some(count) = map_b.insert(c, 1) {
                map_b.insert(c, count + 1);
            }
            if let Some(_) = map_a.get(&c) {
                common = Some(c);
            }
        }

        if let Some(c) = common {
            total_priorities += get_priority(c);
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
