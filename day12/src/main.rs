use std::fs::read_to_string;

mod part1;
use part1::day12_part1;

mod part2;
use part2::day12_part2;

fn main() {
    let text = read_to_string("./rsc/test2")
    .unwrap();

    //day12_part1(&text);
    day12_part2(&text);
}
