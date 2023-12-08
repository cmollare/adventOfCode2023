use std::{fs::read_to_string, cmp::Ordering};

mod part1;
use part1::day8_part1;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day8_part1(&text);
}
