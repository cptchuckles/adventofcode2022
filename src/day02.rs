use std::collections::HashMap;

pub fn execute() {
    let inputs = crate::start_day::setup("02");

    let step1_scores = HashMap::from([
        ("X", HashMap::from([("A", 4), ("B", 1), ("C", 7)])),
        ("Y", HashMap::from([("A", 8), ("B", 5), ("C", 2)])),
        ("Z", HashMap::from([("A", 3), ("B", 9), ("C", 6)])),
    ]);

    let step2_scores = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 4), ("Z", 8)])),
        ("B", HashMap::from([("X", 1), ("Y", 5), ("Z", 9)])),
        ("C", HashMap::from([("X", 2), ("Y", 6), ("Z", 7)])),
    ]);

    let mut step1_total = 0u32;
    let mut step2_total = 0u32;

    for line in inputs {
        let (other, me) = line.split_once(' ').expect("Malformed input");

        step1_total += step1_scores[me][other];
        step2_total += step2_scores[other][me];
    }

    println!("Step 1: {}", step1_total);
    println!("Step 2: {}", step2_total);
}
