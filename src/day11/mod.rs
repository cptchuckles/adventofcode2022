mod monkey;
mod part1;
mod part2;

pub fn execute() {
    let inputs = crate::start_day::setup("11");

    part1::execute(&inputs);
    part2::execute(&inputs);
}
