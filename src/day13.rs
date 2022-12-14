use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Item {
    List(Vec<Item>),
    Number(usize),
}

pub fn execute() {
    let inputs = crate::start_day::setup("13");

    println!("Part 1: {}", part_1(&inputs));

    println!("Part 2: {}", part_2(&inputs));
}

fn part_1(inputs: &Vec<String>) -> usize {
    let mut correct_indices: Vec<usize> = Vec::new();

    for (i, pair) in inputs.split(|s| s.len() == 0).enumerate() {
        let first_line = pair.first().expect("No first list");
        let second_line = pair.last().expect("No second list");
        let first = parse_recursively(first_line).first().unwrap().to_owned();
        let second = parse_recursively(second_line).first().unwrap().to_owned();

        let result = compare_items(&first, &second);

        if result.is_lt() {
            correct_indices.push(i + 1);
        }
    }

    correct_indices.iter().sum()
}

fn part_2(inputs: &Vec<String>) -> usize {
    let mut packets: Vec<Item> = inputs
        .iter()
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }
            Some(parse_recursively(line).first().unwrap().to_owned())
        })
        .collect();

    let two = parse_recursively("[[2]]".into()).first().unwrap().to_owned();
    let six = parse_recursively("[[6]]".into()).first().unwrap().to_owned();
    packets.push(two.to_owned());
    packets.push(six.to_owned());

    packets.sort_by(|a, b| compare_items(a, b));

    let mut itwo = None;

    for (i, packet) in packets.iter().enumerate() {
        if itwo.is_none() && *packet == two {
            itwo = Some(i + 1);
        }
        else if *packet == six {
            return itwo.expect("Must have found itwo by now") * (i + 1);
        }
    }

    unreachable!("How did it come to this?");
}

fn compare_items(first: &Item, second: &Item) -> Ordering {
    match (first, second) {
        // List vs List
        (Item::List(f), Item::List(s)) => {
            let (mut i, mut j) = (f.iter(), s.iter());
            loop {
                let ret = match (i.next(), j.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (Some(a), Some(b)) => compare_items(a, b),
                };
                if ret.is_ne() {
                    return ret;
                }
            }
        }

        // List vs Number
        (Item::List(_), Item::Number(_)) => compare_items(first, &Item::List(vec![second.clone()])),

        // Number vs List
        (Item::Number(_), Item::List(_)) => compare_items(&Item::List(vec![first.clone()]), second),

        // Number vs Number
        (Item::Number(f), Item::Number(s)) => f.cmp(&s),
    }
}

fn parse_recursively(set: &str) -> Vec<Item> {
    if let Ok(n) = set.parse() {
        return vec![Item::Number(n)];
    }

    let mut list: Vec<Item> = Vec::new();

    let mut starti = 0usize;
    let mut setchars = set.chars().enumerate();
    while let Some((i, c)) = setchars.next() {
        match c {
            '[' => {
                let mut depth = 1;
                while let Some((j, c)) = setchars.next() {
                    match c {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                let inner_set = &set[(i + 1)..j];
                                let inner_list = parse_recursively(inner_set);
                                list.push(Item::List(inner_list));
                                starti = j + 1;
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            ',' => {
                if let Ok(n) = set[starti..i].parse() {
                    list.push(Item::Number(n));
                }
                starti = i + 1;
            }
            _ => (),
        }
    }

    if let Ok(n) = set[starti..].parse() {
        list.push(Item::Number(n));
    }

    list
}
