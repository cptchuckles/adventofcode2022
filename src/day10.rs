pub fn execute() {
    let inputs = crate::start_day::setup("10");

    let mut clock = 0i64;
    let mut x = 1i64;

    let mut sum = 0i64;
    let mut crt: [char; 240] = ['.'; 240];

    for input in inputs {
        match input.as_str() {
            "noop" => {
                if tick(&mut clock, &mut crt, x) {
                    sum += clock * x;
                }
            },
            addx => {
                for _ in 0..2 {
                    if tick(&mut clock, &mut crt, x) {
                        sum += clock * x;
                    }
                }
                x += addx.split_once(' ').unwrap().1.parse::<i64>().unwrap();
            },
        }
    }

    println!("Part 1: {}", sum);

    print!("Part 2:");
    print_crt(&crt);
}

fn print_crt(crt: &[char]) {
    for (i, c) in crt.iter().enumerate() {
        if i % 40 == 0 {
            println!();
        }
        print!("{}", c);
    }
    println!();
}

fn tick(clock: &mut i64, crt: &mut [char], x: i64) -> bool {
    let row = *clock / 40;
    let col = *clock - 40 * row;
    if (x - 1) <= col && col <= (x + 1) {
        crt[*clock as usize] = '#';
    }
    *clock += 1;
    (*clock + 20) % 40 == 0
}
