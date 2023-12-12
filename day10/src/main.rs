use std::fs::read_to_string;

mod part1;
use part1::day10_part1;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day10_part1(&text);
}
