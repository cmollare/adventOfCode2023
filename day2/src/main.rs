use std::fs::read_to_string;
use regex::Regex;

mod models;
use models::Game;
use models::Set;
use models::Color;
use models::ColorName;
use models::Guess;

fn main() {
    let text = read_to_string("./rsc/input")
    .unwrap();

    day2_part1(text.clone());
    day2_part2(text);
}

fn day2_part1(s : String) {
    let guess = Guess {
        red: 12,
        green: 13,
        blue: 14
    };

    let res : u64= s.lines()
    .map(String::from)
    .map(split_game)
    .map(|g| compute_valid_ids(&guess, g))
    .sum();

    println!("result guess : {}", res);
}

fn day2_part2(s : String) {

    let res : u64= s.lines()
    .map(String::from)
    .map(split_game)
    .map(|g| g.power)
    .sum();

    println!("result power : {}", res);
}

fn compute_valid_ids(guess: &Guess, g: Game) -> u64 {
    if g.sets.iter().any(|s| s.nb_blue > guess.blue) { return 0 };
    if g.sets.iter().any(|s| s.nb_green > guess.green) { return 0 };
    if g.sets.iter().any(|s| s.nb_red > guess.red) { return 0 };

    return g.id;
}

fn split_game(s: String) -> Game {
    let game : Vec<_>= s.split(":").collect();
    let sets : Vec<_> = game[1]
        .split(";")
        .map(extract_set)
        .collect();

    let cloned_set = sets.to_vec();

    let max_set = cloned_set.iter().map(|s| s.clone()).reduce(compute_max_set).unwrap();
    let power = max_set.nb_blue*max_set.nb_red*max_set.nb_green;

    let re = Regex::new(r"\d+").unwrap();
    let id = re.captures(game[0]).unwrap()[0].parse::<u64>().unwrap();

    return Game {
        id,
        sets,
        power,
    };
}

fn extract_set(s: &str) -> Set {
    return s.split(",")
        .map(extract_color)
        .map(convert_to_partial_set)
        .reduce(compute_max_set).unwrap();
}

fn compute_max_set(s1: Set, s2 : Set) -> Set {
    return Set {
        nb_red: if s1.nb_red > s2.nb_red { s1.nb_red } else { s2.nb_red },
        nb_green: if s1.nb_green > s2.nb_green { s1.nb_green } else { s2.nb_green },
        nb_blue: if s1.nb_blue > s2.nb_blue { s1.nb_blue } else { s2.nb_blue },
    }
}

fn extract_color(s: &str) -> Color {
    let re = Regex::new(r"(blue|green|red)").unwrap();
    let color_name = &re.captures(s).unwrap()[0];
    let color = match color_name {
        "blue" => ColorName::Blue,
        "green" => ColorName::Green,
        "red" => ColorName::Red,
        _ => ColorName::Blank
    };

    let re = Regex::new(r"\d+").unwrap();
    let number = re.captures(s).unwrap()[0].parse::<u64>().unwrap();


    return Color {
        color,
        number
    }
}

fn convert_to_partial_set(c : Color) -> Set {
    
    return match c.color {
        ColorName::Red => Set {
            nb_red: c.number,
            nb_blue: 0,
            nb_green: 0,
        },
        ColorName::Green => Set {
            nb_red: 0,
            nb_blue: 0,
            nb_green: c.number,
        },
        ColorName::Blue => Set {
            nb_red: 0,
            nb_blue: c.number,
            nb_green: 0,
        },
        ColorName::Blank => Set { nb_red: 0, nb_green: 0, nb_blue: 0 }
    }
}