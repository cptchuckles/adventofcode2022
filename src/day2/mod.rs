pub fn execute() {
    crate::headline::print_day(2);

    let lines = crate::slurp::read_lines("inputs/2.txt").expect("Couldn't get day 2 input");

    for line in lines {
        let _text = line.expect("Not a valid string");
    }
}
