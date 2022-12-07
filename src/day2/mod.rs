pub fn execute() {
    crate::headline::print_day(2);

    let lines = crate::slurp::read_lines("src/day2/input.txt").expect("Couldn't get day 2 input");

    for line in lines {
        let _text = line.expect("Not a valid string");
    }
}
