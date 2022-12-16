use std::collections::HashSet;

pub fn execute() {
    let inputs = crate::start_day::setup("09");

    let mut snek = [(0i32, 0i32); 10];

    let mut tail_positions_1: HashSet<(i32, i32)> = HashSet::new();
    let mut tail_positions_2: HashSet<(i32, i32)> = HashSet::new();

    for input in inputs {
        let head_move = match input.split_once(' ').expect("Malformed input") {
            ("R", n) => (0..0, 0..n.parse::<i32>().unwrap(), 1),
            ("L", n) => (0..0, 0..n.parse::<i32>().unwrap(), -1),
            ("U", n) => (0..n.parse::<i32>().unwrap(), 0..0, -1),
            ("D", n) => (0..n.parse::<i32>().unwrap(), 0..0, 1),
            _ => panic!("Illegal move"),
        };

        for _ in head_move.0 {
            snek[0].0 += head_move.2;
            tail_follow(snek[0], &mut snek[1]);
            tail_positions_1.insert(snek[1]);
            for (i, k) in (1..9).zip(2..10) {
                tail_follow(snek[i], &mut snek[k]);
            }
            tail_positions_2.insert(snek[9]);
        }

        for _ in head_move.1 {
            snek[0].1 += head_move.2;
            tail_follow(snek[0], &mut snek[1]);
            tail_positions_1.insert(snek[1]);
            for (i, k) in (1..9).zip(2..10) {
                tail_follow(snek[i], &mut snek[k]);
            }
            tail_positions_2.insert(snek[9]);
        }
    }

    println!("Part 1: {}", tail_positions_1.iter().count());
    println!("Part 2: {}", tail_positions_2.iter().count());
}

fn tail_follow(head: (i32, i32), tail: &mut (i32, i32)) {
    if (head.0 - tail.0).abs() == 2 {
        tail.0 += (head.0 - tail.0).signum();
        if (head.1 - tail.1).abs() > 0 {
            tail.1 += (head.1 - tail.1).signum();
        }
        return;
    }

    if (head.1 - tail.1).abs() == 2 {
        tail.1 += (head.1 - tail.1).signum();
        if (head.0 - tail.0).abs() > 0 {
            tail.0 += (head.0 - tail.0).signum();
        }
    }
}
