pub fn execute() {
    crate::headline::print_day(1);

    let mut sum = 0u64;
    let mut sums: Vec<u64> = Vec::new();

    let lines = crate::slurp::read_lines("src/day1/input.txt").expect("Couldn't get day 1 input");

    for line in lines {
        let text = line.expect("Not a valid string");
        if let Ok(number) = text.parse::<u64>() {
            sum += number;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }

    sums.sort_unstable_by(|a, b| b.cmp(a));
    println!("Part 1: {}", &sums[0]);
    println!("Part 2: {}", sums.iter().take(3).sum::<u64>());
}
