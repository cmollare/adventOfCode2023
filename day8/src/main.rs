use std::{fs::read_to_string, cmp::Ordering};

mod part1;
use part1::day8_part1;

mod part2;
use part2::day8_part2;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    //day8_part1(&text);
    day8_part2(&text);
}
