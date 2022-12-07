use crate::slurp;

pub fn execute() {
    let mut sum = 0u64;
    let mut sums: Vec<u64> = Vec::new();

    let lines = slurp::read_lines("src/day1/input.txt")
        .expect("Couldn't get day 1 input");

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
    println!("Most powerful gnome: {}", &sums[0]);
    println!(
        "Three most powerful gnomes combined: {}",
        sums.iter().take(3).sum::<u64>()
    );
}
