pub fn execute() {
    let inputs = crate::start_day::setup("10");

    let mut clock = 0i64;
    let mut x = 1i64;

    let mut sum = 0i64;

    for input in inputs {
        match input.as_str() {
            "noop" => {
                if tick(&mut clock) {
                    sum += clock * x;
                }
            },
            addx => {
                for _ in 0..2 {
                    if tick(&mut clock) {
                        sum += clock * x;
                    }
                }
                x += addx.split_once(' ').unwrap().1.parse::<i64>().unwrap();
            },
        }
    }

    println!("Part 1: {}", sum);
}

fn tick(clock: &mut i64) -> bool {
    *clock += 1;
    (*clock + 20) % 40 == 0
}
