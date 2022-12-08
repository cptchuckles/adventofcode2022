pub fn execute() {
    let inputs = crate::start_day::setup(4);

    let mut overlaps = 0usize;
    let mut encapsulations = 0usize;

    for line in inputs {
        let (a, b) = line.split_once(',').expect("Malformed input");

        let a = match a.split_once('-') {
            Some((x, y)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()),
            None => panic!("Malformed range"),
        };

        let b = match b.split_once('-') {
            Some((x, y)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()),
            None => panic!("Malformed range"),
        };

        if a.0 <= b.0 && a.1 >= b.1 || b.0 <= a.0 && b.1 >= a.1 {
            encapsulations += 1;
        }

        if b.0 <= a.0 && a.0 <= b.1
            || b.0 <= a.1 && a.1 <= b.1
            || a.0 <= b.0 && b.0 <= a.1
            || a.0 <= b.1 && b.1 <= a.1
        {
            overlaps += 1;
        }
    }

    println!("Part 1: {}", encapsulations);
    println!("Part 2: {}", overlaps);
}
