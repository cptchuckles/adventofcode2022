use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Number(usize),
}

pub fn execute() {
    let inputs = crate::start_day::setup("13");

    let mut correct_indices: Vec<usize> = Vec::new();

    for (i, pair) in inputs.split(|s| s.len() == 0).enumerate() {
        let first_line = pair.first().expect("No first list");
        let second_line = pair.last().expect("No second list");
        let first = parse_recursively(first_line).first().unwrap().clone();
        let second = parse_recursively(second_line).first().unwrap().clone();

        // println!("Found pair:");
        // println!("  {}", first_line);
        // println!("\t{:?}", first);
        // println!("  {}", second_line);
        // println!("\t{:?}", second);

        // println!("Comparing sets:");
        // println!("\t{:?}", first);
        // println!("\t{:?}", second);

        let result = compare_items(first, second);
        // println!("  Result: {:?}", result);
        if result.is_lt() {
            correct_indices.push(i + 1);
        }
    }

    let sum = correct_indices.iter().sum::<usize>();

    // println!("Part 1: {} ({:?})", sum, correct_indices);
    println!("Part 1: {}", sum);
}

fn compare_items(first: Item, second: Item) -> Ordering {
    // println!("    Internal comparing sets:");
    // println!("\t{:?}", first);
    // println!("\t{:?}", second);
    let result = match (first, second) {
        // List vs List
        (Item::List(f), Item::List(s)) => {
            let (mut i, mut j) = (f.iter(), s.iter());
            loop {
                let ret = match (i.next(), j.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => Ordering::Less,
                    (Some(_), None) => Ordering::Greater,
                    (Some(a), Some(b)) => {
                        compare_items(a.clone(), b.clone())
                    }
                };
                if ret.is_eq() {
                    continue;
                }
                return ret;
            }
        }

        // List vs Number
        (Item::List(f), Item::Number(s)) => {
            compare_items(Item::List(f), Item::List(vec![Item::Number(s)]))
        }

        // Number vs List
        (Item::Number(f), Item::List(s)) => {
            compare_items(Item::List(vec![Item::Number(f)]), Item::List(s))
        }

        // Number vs Number
        (Item::Number(f), Item::Number(s)) => f.cmp(&s),
    };
    // println!("\tInterim Result: {:?}", result);
    result
}

fn parse_recursively(set: &str) -> Vec<Item> {
    if let Ok(n) = set.parse::<usize>() {
        return vec![Item::Number(n)];
    }

    let mut list: Vec<Item> = Vec::new();

    let mut i = 0usize;
    let mut starti = 0usize;
    while i < set.len() {
        match set.chars().nth(i).expect("Bounds error") {
            '[' => {
                let mut lookahead = i;
                let mut depth = 1;
                // println!("Set: {}", set);
                while depth > 0 {
                    lookahead += 1;
                    match set.chars().nth(lookahead).expect("Bounds error") {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => (),
                    }
                }
                list.push(Item::List(parse_recursively(&set[(i + 1)..lookahead])));
                i = lookahead;
                starti = lookahead + 1;
            }
            ',' | ']' => {
                if let Ok(n) = set[starti..i].parse::<usize>() {
                    list.push(Item::Number(n));
                }
                starti = i + 1;
            }
            _ => (),
        }
        i += 1;
    }

    if starti < i {
        if let Ok(n) = set[starti..i].parse::<usize>() {
            list.push(Item::Number(n));
        }
    }

    list
}
