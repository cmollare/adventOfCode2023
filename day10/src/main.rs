use std::fs::read_to_string;

mod part1;
use part1::day10_part1;

mod part2;
use part2::day10_part2;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    //day10_part1(&text);
    day10_part2(&text);
}
