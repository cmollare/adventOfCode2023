use std::fs::read_to_string;

mod part1;
use part1::day11_part1;

mod part2;
use part2::day11_part2;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day11_part1(&text);
    day11_part2(&text);
}
